use strum::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum OnError {
    Print,
    Exit,
}
