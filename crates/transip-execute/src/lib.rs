use std::{mem::size_of, time::Duration};

use serde::{Serialize, Serializer};
pub use transip::configuration_from_environment;
use transip::Configuration;
pub use transip::Error;
use transip_command::{
    DnsCommand, DomainCommand, InvoiceAction, InvoiceCommand, OnError, ProductCommand, VpsAction,
    VpsCommand,
};

// reexport TransipCommand
pub use transip_command::TransipCommand;

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
            DnsCommand::AcmeValidationDelete(name) => self
                .inner
                .dns_entry_delete_all(name, DnsEntry::is_acme_challenge)
                .report(s),
            DnsCommand::List(name) => self.inner.dns_entry_list(name).report(s),
            DnsCommand::Delete(name, dns_entry) => dns_entry
                .parse::<DnsEntry>()
                .and_then(|entry| self.inner.dns_entry_delete(name, entry))
                .report(s),
            DnsCommand::Insert(name, dns_entry) => dns_entry
                .parse::<DnsEntry>()
                .and_then(|entry| self.inner.dns_entry_insert(name, entry))
                .report(s),
            DnsCommand::AcmeValidationSet(name, challenge) => self
                .inner
                .dns_entry_delete_all(name, DnsEntry::is_acme_challenge)
                .and_then(|_| {
                    self.inner
                        .dns_entry_insert(name, DnsEntry::new_acme_challenge(60, challenge))
                })
                .report(s),
            #[cfg(feature = "propagation")]
            DnsCommand::AcmeValidationCheck(name, challenge) => {
                acme_validation_propagation::wait(name, challenge).map_err(|_| Error::AcmeChallege)
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
            DomainCommand::Item(name) => self.inner.domain_item(name).report(s),
            DomainCommand::List => self.inner.domain_list().report(s),
        }
    }

    fn execute_invoice(
        &mut self,
        command: &InvoiceCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::account::AccountApi;
        match command {
            InvoiceCommand::Action(number, action) => match action {
                InvoiceAction::Item => self.inner.invoice(number).report(s),
                InvoiceAction::Pdf => self.inner.invoice_pdf(number).report(s),
            },
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
            ProductCommand::Elements(elements) => self.inner.product_elements(elements).report(s),
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
            VpsCommand::Action(name, action) => match action {
                VpsAction::Item => self.inner.vps(name).report(s),
                VpsAction::Lock => self.inner.vps_set_is_locked(name, true).report(s),
                VpsAction::Reset => self.inner.vps_reset(name).report(s),
                VpsAction::Start => self.inner.vps_start(name).report(s),
                VpsAction::Stop => self.inner.vps_stop(name).report(s),
                VpsAction::Unlock => self.inner.vps_set_is_locked(name, false).report(s),
            },
            VpsCommand::List => self.inner.vps_list().report(s),
        }
    }

    pub fn execute(
        &mut self,
        command: &TransipCommand,
        s: impl Serializer,
    ) -> Result<(), transip::Error> {
        use transip::api::general::GeneralApi;
        match command {
            TransipCommand::Comment(_) => Ok(()),
            TransipCommand::Dns(command) => self.execute_dns(command, s),
            TransipCommand::Domain(command) => self.execute_domain(command, s),
            TransipCommand::Invoice(command) => self.execute_invoice(command, s),
            TransipCommand::OnError(onerror) => {
                self.onerror = onerror.clone();
                Ok(())
            }
            TransipCommand::Ping => self.inner.api_test().report(s),
            TransipCommand::Product(command) => self.execute_product(command, s),
            TransipCommand::Sleep(timeout) => {
                std::thread::sleep(Duration::from_secs(*timeout));
                Ok(())
            }
            TransipCommand::Vps(command) => self.execute_vps(command, s),
            TransipCommand::AvailibilityZones => self.inner.availability_zones().report(s),
        }
    }
}
