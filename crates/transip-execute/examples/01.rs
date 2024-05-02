use std::error::Error;

use transip::configuration_from_environment;
use transip_command::{SubCommand, TransipCommand};
use transip_execute::Client;

const COMMAND_DNS_LIST: &str = "dns list paulmin.nl";
const COMMAND_ADD_CHALLENGE: &str = "dns acme-validation-set paulmin.nl 89823875";
const COMMAND_DELETE_CHALLENGE: &str = "dns acme-validation-delete paulmin.nl";

enum Out {
    Json,
}

macro_rules! execute_out {
    ($ser:path, $client:ident, $command:ident, $print:ident) => {
        let mut buffer: Vec<u8> = Vec::new();
        let mut ser = $ser(&mut buffer);

        match $client.execute($command, &mut ser) {
            Ok(_) => {
                let s = String::from_utf8(buffer).unwrap();
                if s.len() > 0 {
                    $print!("{}", s);
                }
            }
            Err(error) => eprintln!("Error: {error}"),
        }
    };
}

impl Out {
    fn execute(&self, client: &mut Client, command: &SubCommand) {
        match self {
            Out::Json => {
                execute_out!(serde_json::Serializer::pretty, client, command, println);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client: Client = configuration_from_environment().and_then(Client::try_from)?;

    let command_dns_list = COMMAND_DNS_LIST.parse::<TransipCommand>()?;
    let command_add_challenge = COMMAND_ADD_CHALLENGE.parse::<TransipCommand>()?;
    let command_delete_challenge = COMMAND_DELETE_CHALLENGE.parse::<TransipCommand>()?;

    Out::Json.execute(&mut client, &command_dns_list.command);
    Out::Json.execute(&mut client, &command_dns_list.command);
    Out::Json.execute(&mut client, &command_delete_challenge.command);
    Out::Json.execute(&mut client, &command_add_challenge.command);

    Ok(())
}
