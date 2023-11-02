use serde::Serialize;
use transip::{
    api::{
        account::AccountApi,
        dns::{DnsApi, DnsEntry},
        domain::DomainApi,
        general::GeneralApi,
        vps::VpsApi,
    },
    Client,
};

use crate::command::{
    dns::DnsCommand,
    domain::DomainCommand,
    invoice::{InvoiceAction, InvoiceCommand},
    product::ProductCommand,
    vps::{VpsAction, VpsCommand},
    TransipCommand,
};

pub trait ToJson {
    fn and_then_json(self) -> Result<String, transip::Error>;
}

impl<T: Serialize> ToJson for Result<T, transip::Error> {
    fn and_then_json(self) -> Result<String, transip::Error> {
        self.and_then(|t| serde_json::to_string_pretty(&t).map_err(transip::Error::from))
    }
}

pub trait Execution {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error>;
}

impl Execution for TransipCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::Comment(_comment) => Ok(()).and_then_json(),
            Self::Dns(command) => command.execute(client),
            Self::Domain(command) => command.execute(client),
            Self::Invoice(command) => command.execute(client),
            Self::Product(command) => command.execute(client),
            Self::Vps(command) => command.execute(client),
        }
    }
}

impl Execution for DnsCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::DeleteAcmeChallenge(name) => {
                DnsApi::dns_entry_delete_all(client, name, DnsEntry::is_acme_challenge)
                    .and_then_json()
            }
            Self::List(name) => DnsApi::dns_entry_list(client, name).and_then_json(),
            Self::SetAcmeChallenge(name, value) => {
                DnsApi::dns_entry_delete_all(client, name, DnsEntry::is_acme_challenge)
                    .and_then(|_| {
                        DnsApi::dns_entry_insert(
                            client,
                            name,
                            DnsEntry::new_acme_challenge(60, value),
                        )
                    })
                    .and_then_json()
            }
        }
    }
}

impl Execution for DomainCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::Item(name) => client.domain_item(name).and_then_json(),
            Self::List => client.domain_list().and_then_json(),
        }
    }
}

impl Execution for InvoiceCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::Action(name, action) => match action {
                InvoiceAction::Item => client.invoice(name).and_then_json(),
                InvoiceAction::Pdf => client.invoice_pdf(name).and_then_json(),
            },
            Self::List => client.invoice_list().and_then_json(),
        }
    }
}

impl Execution for ProductCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::Elements(elements) => client.product_elements(elements).and_then_json(),
            Self::List => client.products().and_then_json(),
        }
    }
}

impl Execution for VpsCommand {
    fn execute(&self, client: &mut Client) -> Result<String, transip::Error> {
        match self {
            Self::Action(name, action) => match action {
                VpsAction::Item => client.vps(name).and_then_json(),
                VpsAction::Lock => client.vps_set_is_locked(name, true).and_then_json(),
                VpsAction::Reset => client.vps_reset(name).and_then_json(),
                VpsAction::Start => client.vps_start(name).and_then_json(),
                VpsAction::Stop => client.vps_stop(name).and_then_json(),
                VpsAction::Unlock => client.vps_set_is_locked(name, false).and_then_json(),
            },
            Self::List => client.vps_list().and_then_json(),
        }
    }
}
