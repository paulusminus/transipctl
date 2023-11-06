use crate::{check_environment, error::Error, str_extension::StrExtension};
use std::{fmt::Display, str::FromStr};
use strum::{Display, EnumString};

pub type VpsName = String;

const LIST: &str = "list";

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum VpsAction {
    Item,
    Lock,
    Unlock,
    Start,
    Stop,
    Reset,
}

#[derive(Debug, PartialEq)]
pub enum VpsCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{VpsCommand, TransipCommand};
    ///
    /// let commandline = "vps list";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Vps(VpsCommand::List),
    /// );
    /// ```
    List,

    /// # Example
    ///
    /// ```
    /// use transip_command::{TransipCommand, VpsAction, VpsCommand};
    ///
    /// let commandline = "vps reset vps9374";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Vps(
    ///         VpsCommand::Action(
    ///             "vps9374".to_owned(),
    ///             VpsAction::Reset,
    ///         )
    ///     ),
    /// );
    /// ```
    Action(VpsName, VpsAction),
}

impl Display for VpsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VpsCommand::Action(name, action) => write!(f, "{} {}", action, name),
            VpsCommand::List => write!(f, "{}", LIST),
        }
    }
}

impl FromStr for VpsCommand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim() == LIST {
            return Ok(VpsCommand::List);
        }

        for action in [
            VpsAction::Item,
            VpsAction::Lock,
            VpsAction::Reset,
            VpsAction::Start,
            VpsAction::Stop,
            VpsAction::Unlock,
        ] {
            if let Some(vps_name) = s.one_param(action.to_string().as_str()) {
                return Ok(VpsCommand::Action(check_environment(vps_name)?, action));
            }
        }

        Err(Error::ParseVpsCommand(s.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::{VpsAction, VpsCommand};

    #[test]
    fn display() {
        assert_eq!(
            VpsCommand::Action("vps2".to_owned(), VpsAction::Start).to_string(),
            "start vps2".to_owned(),
        );

        assert_eq!(VpsCommand::List.to_string(), "list".to_owned(),);
    }
}
