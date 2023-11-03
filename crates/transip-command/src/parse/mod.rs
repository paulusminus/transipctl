use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parse/transip.pest"]
pub(crate) struct TransipCommandParser;
