use transip::configuration_from_environment;
use transip_command::TransipCommand;
use transip_execute::Client;

const COMMAND_DNS_LIST: &str = "dns list paulmin.nl";
const COMMAND_ADD_CHALLENGE: &str = "dns acme-challenge-set paulmin.nl 89823875";
const COMMAND_DELETE_CHALLENGE: &str = "dns acme-challenge-delete paulmin.nl";

enum Out {
    Json,
    Yaml,
}

fn execute(client: &mut Client, command: &TransipCommand, out: Out) {
    match out {
        Out::Json => {
            let mut buffer: Vec<u8> = Vec::new();
            let mut ser = serde_json::Serializer::pretty(&mut buffer);

            match client.execute(command, &mut ser) {
                Ok(_) => {
                    let s = String::from_utf8(buffer).unwrap();
                    if s.len() > 0 {
                        println!("{}", s);
                    }
                }
                Err(error) => eprintln!("Error: {error}"),
            }
        }
        Out::Yaml => {
            let mut buffer: Vec<u8> = Vec::new();
            let mut ser = serde_yaml::Serializer::new(&mut buffer);

            match client.execute(command, &mut ser) {
                Ok(_) => {
                    let s = String::from_utf8(buffer).unwrap();
                    if s.len() > 0 {
                        print!("{}", s);
                    }
                }
                Err(error) => eprintln!("Error: {error}"),
            }
        }
    }
}

fn main() {
    let mut client: Client = configuration_from_environment()
        .and_then(Client::try_from)
        .expect("Cliënt failed");

    let command_dns_list = COMMAND_DNS_LIST
        .parse::<TransipCommand>()
        .expect("Parse failed");

    let command_add_challenge = COMMAND_ADD_CHALLENGE
        .parse::<TransipCommand>()
        .expect("Parse failed");

    let command_delete_challenge = COMMAND_DELETE_CHALLENGE
        .parse::<TransipCommand>()
        .expect("Parse failed");

    execute(&mut client, &command_dns_list, Out::Json);
    execute(&mut client, &command_dns_list, Out::Yaml);
    execute(&mut client, &command_delete_challenge, Out::Json);
    execute(&mut client, &command_add_challenge, Out::Json);
}
