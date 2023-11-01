use pest::iterators::Pair;
use strum::EnumString;
use transip::{api::account::AccountApi, Client};

use crate::{
    error::{Error, ErrorExt},
    to_json, Result, Rule,
};

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum InvoiceAction {
    Item,
    Pdf,
}

pub type InvoiceNumber = String;

#[derive(Debug, PartialEq)]
pub enum InvoiceCommand {
    List,
    Action(InvoiceNumber, InvoiceAction),
}

impl<'a> TryFrom<Pair<'a, Rule>> for InvoiceCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> std::result::Result<Self, Self::Error> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::invoice_list => Ok(Self::List),
            Rule::invoice_item_action => {
                let mut inner = inner.into_inner();
                let action = inner.next().unwrap().as_str().trim();
                let name = inner.next().unwrap().as_str().trim();
                action.parse::<InvoiceAction>()
                .err_into()
                .map(|action| Self::Action(name.to_owned(), action))
            }
            _ => Err(Error::ParseInvoiceCommand(commandline)),
        }
            
    }
}

pub fn execute(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
    let commandline = pair.as_str().to_owned();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::invoice_list => client.invoice_list().err_into().and_then(to_json),
        Rule::invoice_item_action => {
            let mut inner = inner.into_inner();
            let action = inner.next().unwrap().as_str().trim();
            let name = inner.next().unwrap().as_str().trim();
            if action == "item" {
                client.invoice(name).err_into().and_then(to_json)
            } else if action == "pdf" {
                client.invoice_pdf(name).err_into().and_then(to_json)
            } else {
                Err(Error::ParseInvoiceCommand(commandline))
            }
        }
        _ => Err(Error::ParseInvoiceCommand(commandline)),
    }
}
