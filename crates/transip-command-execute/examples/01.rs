use std::io::stdout;

use transip::configuration_from_environment;
use transip_command::TransipCommand;
use transip_command_execute::Client;

const COMMAND_DNS_LIST: &str = "dns list paulmin.nl";
// const COMMAND_ADD_CHALLENGE: &str = "dns acme-challenge-set paulmin.nl 89823875";
const COMMAND_DELETE_CHALLENGE: &str = "dns acme-challenge-delete paulmin.nl";

fn main() {
    let mut client: Client = configuration_from_environment()
        .and_then(Client::try_from)
        .expect("Cliënt failed");

    let command_dns_list = COMMAND_DNS_LIST
        .parse::<TransipCommand>()
        .expect("Parse failed");

    // let command_add_challenge = COMMAND_ADD_CHALLENGE.parse::<TransipCommand>()
    //     .expect("Parse failed");

    let command_delete_challenge = COMMAND_DELETE_CHALLENGE
        .parse::<TransipCommand>()
        .expect("Parse failed");

    // let mut json = serde_json::Serializer::pretty(stdout());

    let mut yaml = serde_yaml::Serializer::new(stdout());

    client.execute(command_dns_list, &mut yaml);
    // client.execute(command_add_challenge, &mut s);
    client.execute(command_delete_challenge, &mut yaml);
}
