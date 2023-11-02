use transip::{api::account::AccountApi, Client};

use crate::{
    command::invoice::{InvoiceAction, InvoiceCommand},
    Result,
};

use super::{Execution, ToJson};

impl Execution for InvoiceCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
        match self {
            Self::Action(name, action) => match action {
                InvoiceAction::Item => client.invoice(name).and_then_json(),
                InvoiceAction::Pdf => client.invoice_pdf(name).and_then_json(),
            },
            Self::List => client.invoice_list().and_then_json(),
        }
    }
}
