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

pub trait Execution {
    fn execute(&self, client: &mut Client);
}

impl Execution for TransipCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::Comment(_comment) => (),
            Self::Dns(command) => command.execute(client),
            Self::Domain(command) => command.execute(client),
            Self::Invoice(command) => command.execute(client),
            Self::Product(command) => command.execute(client),
            Self::Vps(command) => command.execute(client),
        }
    }
}

impl Execution for DnsCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::DeleteAcmeChallenge(name) => {
                DnsApi::dns_entry_delete_all(client, name, DnsEntry::is_acme_challenge)
                    .json_report()
            }
            Self::List(name) => DnsApi::dns_entry_list(client, name).json_report(),
            Self::SetAcmeChallenge(name, value) => {
                DnsApi::dns_entry_delete_all(client, name, DnsEntry::is_acme_challenge)
                    .and_then(|_| {
                        DnsApi::dns_entry_insert(
                            client,
                            name,
                            DnsEntry::new_acme_challenge(60, value),
                        )
                    })
                    .json_report()
            }
        }
    }
}

impl Execution for DomainCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::Item(name) => client.domain_item(name).json_report(),
            Self::List => client.domain_list().json_report(),
        }
    }
}

impl Execution for InvoiceCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::Action(name, action) => match action {
                InvoiceAction::Item => client.invoice(name).json_report(),
                InvoiceAction::Pdf => client.invoice_pdf(name).json_report(),
            },
            Self::List => client.invoice_list().json_report(),
        }
    }
}

impl Execution for ProductCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::Elements(elements) => client.product_elements(elements).json_report(),
            Self::List => client.products().json_report(),
        }
    }
}

pub trait JsonReport {
    fn json_report(self);
}

impl<T: Serialize> JsonReport for std::result::Result<T, transip::Error> {
    fn json_report(self) {
        match self {
            Ok(t) => {
                let s = serde_json::to_string_pretty(&t).unwrap();
                if s != "null" {
                    println!("{}", s);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }
}

impl Execution for VpsCommand {
    fn execute(&self, client: &mut Client) {
        match self {
            Self::Action(name, action) => match action {
                VpsAction::Item => client.vps_list().json_report(),
                VpsAction::Lock => client.vps_set_is_locked(name, true).json_report(),
                VpsAction::Reset => client.vps_reset(name).json_report(),
                VpsAction::Start => client.vps_start(name).json_report(),
                VpsAction::Stop => client.vps_stop(name).json_report(),
                VpsAction::Unlock => client.vps_set_is_locked(name, false).json_report(),
            },
            Self::List => client.vps_list().json_report(),
        }
    }
}
