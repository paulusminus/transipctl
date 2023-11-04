use transip::configuration_from_environment;
use transip_command::TransipCommand;
use transip_execute::Client;

const COMMAND_DNS_LIST: &str = "dns list paulmin.nl";
// const COMMAND_ADD_CHALLENGE: &str = "dns acme-challenge-set paulmin.nl 89823875";
const COMMAND_DELETE_CHALLENGE: &str = "dns acme-challenge-delete paulmin.nl";

fn execute(client: &mut Client, command: &TransipCommand) {
    let mut yaml = serde_yaml::Serializer::new(Vec::with_capacity(128));

    match client.execute(command, &mut yaml) {
        Ok(_) => {
            let s = String::from_utf8(yaml.into_inner().unwrap()).unwrap();
            println!("{}", s);
        },
        Err(error) => eprintln!("Error: {error}"),
    }
}

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
    
    execute(&mut client, &command_dns_list);
    execute(&mut client, &command_delete_challenge);
}
