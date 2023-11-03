use serde::Serialize;
use transip::Client;
use transip_command::TransipCommand;

use crate::{error::ErrorExt, Result};

pub mod dns;
pub mod domain;
pub mod invoice;
pub mod product;
pub mod vps;

pub trait ToJson {
    fn and_then_json(self) -> Result<String>;
}

impl<T: Serialize> ToJson for std::result::Result<T, transip::Error> {
    fn and_then_json(self) -> Result<String> {
        self.err_into()
            .and_then(|t| serde_yaml::to_string(&t).err_into())
    }
}

pub trait Execution {
    fn execute(&self, client: &mut Client) -> Result<String>;
}

pub fn execute(command: TransipCommand, client: &mut Client) -> Result<String> {
    match command {
        TransipCommand::Comment(_comment) => Ok(()).and_then_json(),
        TransipCommand::Dns(command) => dns::execute(command, client),
        TransipCommand::Domain(command) => domain::execute(command, client),
        TransipCommand::Invoice(command) => invoice::execute(command, client),
        TransipCommand::Product(command) => product::execute(command, client),
        TransipCommand::Vps(command) => vps::execute(command, client),
    }
}

#[cfg(test)]
mod test {
    use super::ToJson;

    #[test]
    fn serde_yaml() {
        let s = Ok(()).and_then_json().unwrap();
        assert_eq!(s.as_str(), "null\n")
    }
}
