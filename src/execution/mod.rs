use serde::Serialize;
use transip::Client;

use crate::{command::TransipCommand, error::ErrorExt, Result};

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

impl Execution for TransipCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
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
