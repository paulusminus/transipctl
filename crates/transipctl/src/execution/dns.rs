use super::ToJson;
use crate::Result;
use transip::{
    api::dns::{DnsApi, DnsEntry},
    Client,
};
use transip_command::dns::DnsCommand;

pub fn execute(command: DnsCommand, client: &mut Client) -> Result<String> {
    match command {
        DnsCommand::DeleteAcmeChallenge(name) => {
            DnsApi::dns_entry_delete_all(client, &name, DnsEntry::is_acme_challenge).and_then_json()
        }
        DnsCommand::List(name) => DnsApi::dns_entry_list(client, &name).and_then_json(),
        DnsCommand::SetAcmeChallenge(name, value) => {
            DnsApi::dns_entry_delete_all(client, &name, DnsEntry::is_acme_challenge)
                .and_then(|_| {
                    DnsApi::dns_entry_insert(
                        client,
                        &name,
                        DnsEntry::new_acme_challenge(60, &value),
                    )
                })
                .and_then_json()
        }
    }
}
