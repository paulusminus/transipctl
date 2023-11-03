use super::ToJson;
use crate::Result;
use transip::{api::account::AccountApi, Client};
use transip_command::invoice::{InvoiceAction, InvoiceCommand};

pub fn execute(command: InvoiceCommand, client: &mut Client) -> Result<String> {
    match command {
        InvoiceCommand::Action(name, action) => match action {
            InvoiceAction::Item => client.invoice(&name).and_then_json(),
            InvoiceAction::Pdf => client.invoice_pdf(&name).and_then_json(),
        },
        InvoiceCommand::List => client.invoice_list().and_then_json(),
    }
}
