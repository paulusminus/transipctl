use serde::Serialize;
use transip::Client;

use crate::{command::TransipCommand, error::{Error, ErrorExt}};

pub mod dns;
pub mod domain;
pub mod invoice;
pub mod product;
pub mod vps;

pub trait ToJson {
    fn and_then_json(self) -> Result<String, Error>;
}

impl<T: Serialize> ToJson for Result<T, transip::Error> {
    fn and_then_json(self) -> Result<String, Error> {
        self.err_into().and_then(|t| serde_json::to_string_pretty(&t).err_into())
    }
}

pub trait Execution {
    fn execute(&self, client: &mut Client) -> Result<String, Error>;
}

impl Execution for TransipCommand {
    fn execute(&self, client: &mut Client) -> Result<String, Error> {
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
