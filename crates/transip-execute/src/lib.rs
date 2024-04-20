use std::{mem::size_of, time::Duration};

use serde::{Serialize, Serializer};
pub use transip::configuration_from_environment;
pub use transip::Error;
use transip::{api::email::MailForwardInsert, Configuration};
use transip_command::{
    DnsCommand, DomainCommand, EmailBoxCommand, EmailForwardCommand, InvoiceCommand, OnError,
    ProductCommand, VpsCommand,
};

// reexport TransipCommand
pub use transip_command::{ErrorKind, SubCommand, TransipCommand};

pub struct Client {
    inner: transip::Client,
    onerror: transip_command::OnError,
}

impl Client {
    pub fn exit_on_error(&self) -> bool {
        self.onerror == OnError::Exit
    }
}

trait Report {
    fn report(self, s: impl Serializer) -> Result<(), transip::Error>;
}

impl<T: Serialize> Report for Result<T, transip::Error> {
    fn report(self, s: impl Serializer) -> Result<(), transip::Error> {
        self.map(|result| {
            if size_of::<T>() > 0 {
                result.serialize(s).unwrap();
            }
        })
    }
}

impl TryFrom<Box<dyn Configuration>> for Client {
    type Error = transip::Error;

    fn try_from(configuration: Box<dyn Configuration>) -> Result<Self, Self::Error> {
        transip::Client::try_from(configuration).map(|client| Client {
            inner: client,
            onerror: OnError::Print,
        })
    }
}

impl Client {
    fn execute_dns(
        &mut self,
        command: &DnsCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::dns::{DnsApi, DnsEntry};
        match command {
            DnsCommand::AcmeValidationDelete { domain } => self
                .inner
                .dns_entry_delete_all(domain, DnsEntry::is_acme_challenge)
                .report(s),
            DnsCommand::List { domain } => self.inner.dns_entry_list(domain).report(s),
            DnsCommand::Delete(dns_entry) => {
                let entry = DnsEntry {
                    name: dns_entry.name.clone(),
                    expire: dns_entry.ttl,
                    entry_type: format!("{:?}", dns_entry.r#type),
                    content: dns_entry.content.clone(),
                };
                self.inner
                    .dns_entry_delete(&dns_entry.domain, entry)
                    .report(s)
            }
            DnsCommand::Insert(dns_entry) => {
                let entry = DnsEntry {
                    name: dns_entry.name.clone(),
                    expire: dns_entry.ttl,
                    entry_type: format!("{:?}", dns_entry.r#type),
                    content: dns_entry.content.clone(),
                };
                self.inner
                    .dns_entry_insert(&dns_entry.domain, entry)
                    .report(s)
            }
            DnsCommand::AcmeValidationSet { domain, challenge } => self
                .inner
                .dns_entry_delete_all(domain, DnsEntry::is_acme_challenge)
                .and_then(|_| {
                    self.inner
                        .dns_entry_insert(domain, DnsEntry::new_acme_challenge(60, challenge))
                })
                .report(s),
            #[cfg(feature = "propagation")]
            DnsCommand::AcmeValidationCheck { domain, challenge } => {
                acme_validation_propagation::wait(domain, challenge)
                    .map_err(|_| Error::AcmeChallege)
            }
        }
    }

    fn execute_domain(
        &mut self,
        command: &DomainCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::domain::DomainApi;
        match command {
            DomainCommand::Item { domain } => self.inner.domain_item(domain).report(s),
            DomainCommand::List => self.inner.domain_list().report(s),
        }
    }

    fn execute_email_box(
        &mut self,
        command: &EmailBoxCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::email::EmailApi;
        match command {
            EmailBoxCommand::Item { domain, id } => self.inner.mailbox_item(domain, id).report(s),
            EmailBoxCommand::List { domain } => self.inner.mailbox_list(domain).report(s),
            _ => Ok(()),
        }
    }

    fn execute_email_forward(
        &mut self,
        command: &EmailForwardCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::email::EmailApi;
        match command {
            EmailForwardCommand::Item { domain, id } => {
                self.inner.mailforward_item(domain, id).report(s)
            }
            EmailForwardCommand::List { domain } => self.inner.mailforward_list(domain).report(s),
            EmailForwardCommand::Insert {
                domain,
                local_part,
                forward_to,
            } => {
                let mail_forward = MailForwardInsert {
                    local_part: local_part.clone(),
                    forward_to: forward_to.clone(),
                };
                self.inner.mailforward_insert(domain, mail_forward)
            }
            EmailForwardCommand::Delete { domain, id } => self.inner.mailforward_delete(domain, id),
        }
    }

    fn execute_invoice(
        &mut self,
        command: &InvoiceCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::account::AccountApi;
        match command {
            InvoiceCommand::Item { number } => self.inner.invoice(number).report(s),
            InvoiceCommand::Pdf { number } => self.inner.invoice_pdf(number).report(s),
            InvoiceCommand::List => self.inner.invoice_list().report(s),
        }
    }

    fn execute_product(
        &mut self,
        command: &ProductCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::general::GeneralApi;
        match command {
            ProductCommand::Elements { name } => self.inner.product_elements(name).report(s),
            ProductCommand::List => self.inner.products().report(s),
        }
    }

    fn execute_vps(
        &mut self,
        command: &VpsCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::vps::VpsApi;
        match command {
            VpsCommand::Item { name } => self.inner.vps(name).report(s),
            VpsCommand::Lock { name } => self.inner.vps_set_is_locked(name, true).report(s),
            VpsCommand::Reset { name } => self.inner.vps_reset(name).report(s),
            VpsCommand::Start { name } => self.inner.vps_start(name).report(s),
            VpsCommand::Stop { name } => self.inner.vps_stop(name).report(s),
            VpsCommand::Unlock { name } => self.inner.vps_set_is_locked(name, false).report(s),
            VpsCommand::List => self.inner.vps_list().report(s),
        }
    }

    pub fn execute(
        &mut self,
        command: &SubCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::general::GeneralApi;
        match command {
            SubCommand::AvailibilityZones => self.inner.availability_zones().report(s),
            SubCommand::Comment { text: _ } => Ok(()),
            SubCommand::Dns(command) => self.execute_dns(command, s),
            SubCommand::Domain(command) => self.execute_domain(command, s),
            SubCommand::EmailBox(command) => self.execute_email_box(command, s),
            SubCommand::EmailForward(command) => self.execute_email_forward(command, s),
            SubCommand::Invoice(command) => self.execute_invoice(command, s),
            SubCommand::Onerror { on_error } => {
                self.onerror = on_error.clone();
                Ok(())
            }
            SubCommand::Ping => self.inner.api_test().report(s),
            SubCommand::Product(command) => self.execute_product(command, s),
            SubCommand::Sleep { number_of_seconds } => {
                std::thread::sleep(Duration::from_secs(*number_of_seconds));
                Ok(())
            }
            SubCommand::Vps(command) => self.execute_vps(command, s),
            // _ => Ok(()),
        }
    }
}
