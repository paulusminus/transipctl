use super::{Execution, ToJson};
use crate::{command::dns::DnsCommand, Result};
use transip::{
    api::dns::{DnsApi, DnsEntry},
    Client,
};

impl Execution for DnsCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
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
