use serde::{Serialize, Serializer};
use transip::Configuration;
use transip_command::{TransipCommand, DnsCommand, DomainCommand, InvoiceCommand, InvoiceAction, ProductCommand, VpsCommand, VpsAction};
pub use transip::configuration_from_environment;
pub use transip::Error;

pub struct Client {
    inner: transip::Client,
}

trait Report {
    fn report(self, s: impl Serializer);
    fn report_error(self);
}

impl<T: Serialize> Report for Result<T, transip::Error> {
    fn report(self, s: impl Serializer) {
        match self {
            Ok(result) => {
                if let Err(error) = result.serialize(s) {
                    eprintln!("Error: {}", error)
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    fn report_error(self) {
        if let Err(error) = self {
            eprintln!("Error: {error}");
        }
    }
}

impl TryFrom<Box<dyn Configuration>> for Client {
    type Error = transip::Error;

    fn try_from(configuration: Box<dyn Configuration>) -> Result<Self, Self::Error> {
        transip::Client::try_from(configuration)
        .map(|client| Client { inner: client })
    }
}

impl Client {
    fn execute_dns(&mut self, command: DnsCommand, s: impl Serializer) {
        use transip::api::dns::{DnsApi, DnsEntry};
        match command {
            DnsCommand::DeleteAcmeChallenge(name) => {
                self.inner.dns_entry_delete_all(&name, DnsEntry::is_acme_challenge)
                .report_error()
            }
            DnsCommand::List(name) => {
                self.inner.dns_entry_list(&name)
                .report(s)
            }
            DnsCommand::SetAcmeChallenge(name, challenge) => {
                self.inner.dns_entry_delete_all(&name, DnsEntry::is_acme_challenge)
                .and_then(|_| self.inner.dns_entry_insert(&name, DnsEntry::new_acme_challenge(60, &challenge)))
                .report_error()
            }
        }
    }

    fn execute_domain(&mut self, command: DomainCommand, s: impl Serializer) {
        use transip::api::domain::DomainApi;
        match command {
            DomainCommand::Item(name) => {
                self.inner.domain_item(&name).report(s);
            }
            DomainCommand::List => {
                self.inner.domain_list().report(s);
            }
        }
    }

    fn execute_invoice(&mut self, command: InvoiceCommand, s: impl Serializer) {
        use transip::api::account::AccountApi;
        match command {
            InvoiceCommand::Action(number, action) => {
                match action {
                    InvoiceAction::Item => {
                        self.inner.invoice(&number).report(s);
                    }
                    InvoiceAction::Pdf => {
                        self.inner.invoice_pdf(&number).report(s);
                    }
                }
            }
            InvoiceCommand::List => {
                self.inner.invoice_list().report(s);
            }
        }
    }

    fn execute_product(&mut self, command: ProductCommand, s: impl Serializer) {
        use transip::api::general::GeneralApi;
        match command {
            ProductCommand::Elements(elements) => {
                self.inner.product_elements(&elements).report(s);
            }
            ProductCommand::List => {
                self.inner.products().report(s);
            }
        }
    }

    fn execute_vps(&mut self, command: VpsCommand, s: impl Serializer) {
        use transip::api::vps::VpsApi;
        match command {
            VpsCommand::Action(name, action) => {
                match action {
                    VpsAction::Item => {
                        self.inner.vps(&name).report(s);
                    }
                    VpsAction::Lock => {
                        self.inner.vps_set_is_locked(&name, true).report_error();
                    }
                    VpsAction::Reset => {
                        self.inner.vps_reset(&name).report_error();
                    }
                    VpsAction::Start => {
                        self.inner.vps_start(&name).report_error();
                    }
                    VpsAction::Stop => {
                        self.inner.vps_stop(&name).report_error();
                    }
                    VpsAction::Unlock => {
                        self.inner.vps_set_is_locked(&name, false).report_error();
                    }
                }
            }
            VpsCommand::List => {
                self.inner.vps_list().report(s);
            }
        }
    }

    pub fn execute(&mut self, command: TransipCommand, s: impl Serializer)
    {
        match command {
            TransipCommand::Comment(_) => (),
            TransipCommand::Dns(command) => self.execute_dns(command, s),
            TransipCommand::Domain(command) => self.execute_domain(command, s),
            TransipCommand::Invoice(command) => self.execute_invoice(command, s),
            TransipCommand::Product(command) => self.execute_product(command, s),
            TransipCommand::Vps(command) => self.execute_vps(command, s),
        }
    }
}