use std::{fmt::Display, str::FromStr};
use crate::{error::Error, str_extension::StrExtension, check_environment};

pub type DomainName = String;

const ACME_VALIDATION_DELETE: &str = "acme-validation-delete";
const ACME_VALIDATION_SET: &str = "acme-validation-set";
const LIST: &str = "list";


#[derive(Debug, PartialEq)]
pub enum DnsCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns list lkdjfie.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::List(
    ///             "lkdjfie.nl".to_owned()
    ///         )
    ///     ),
    /// );
    /// ```
    List(DomainName),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand2};
    ///
    /// let commandline = "dns acme-validation-delete lkdfjf.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand2>().unwrap(),
    ///     TransipCommand2::Dns(
    ///         DnsCommand::AcmeValidationDelete(
    ///             "lkdfjf.nl".to_owned()
    ///         )
    ///     ),
    /// );
    /// ```
    AcmeValidationDelete(DomainName),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand2};
    ///
    /// let commandline = "dns acme-validation-set lkdfjf.nl oe8rtg";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand2>().unwrap(),
    ///     TransipCommand2::Dns(
    ///         DnsCommand::AcmeValidationSet(
    ///             "lkdfjf.nl".to_owned(),
    ///             "oe8rtg".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    AcmeValidationSet(DomainName, String),
}

impl Display for DnsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsCommand::AcmeValidationDelete(name) => write!(f, "{} {}", ACME_VALIDATION_DELETE, name),
            DnsCommand::List(name) => write!(f, "{} {}", LIST, name),
            DnsCommand::AcmeValidationSet(name, challenge) => {
                write!(f, "{} {} {}", ACME_VALIDATION_SET, name, challenge)
            }
        }
    }
}

impl FromStr for DnsCommand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        if let Some(domain_name) = s.one_param(ACME_VALIDATION_DELETE) {
            return Ok(DnsCommand::AcmeValidationDelete(check_environment(domain_name)?));
        }
        if let Some((domain_name, challenge)) = s.two_params(ACME_VALIDATION_SET) {
            return Ok(DnsCommand::AcmeValidationSet(check_environment(domain_name)?, check_environment(challenge)?));
        }
        if let Some(domain_name) = s.one_param(LIST) {
            return Ok(DnsCommand::List(check_environment(domain_name)?));
        }
        Err(Error::ParseDnsCommand(s.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::DnsCommand;

    #[test]
    fn display() {
        assert_eq!(
            DnsCommand::List("paulmin.nl".to_owned()).to_string(),
            "list paulmin.nl".to_owned(),
        );

        assert_eq!(
            DnsCommand::AcmeValidationDelete("paulmin.nl".to_owned()).to_string(),
            "acme-validation-delete paulmin.nl".to_owned(),
        );

        assert_eq!(
            DnsCommand::AcmeValidationSet("paulmin.nl".to_owned(), "hallo".to_owned()).to_string(),
            "acme-validation-set paulmin.nl hallo".to_owned(),
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "acme-validation-set ${CERTBOT_DOMAIN} ${CERTBOT_VALIDATION}".parse::<DnsCommand>().unwrap(),
            DnsCommand::AcmeValidationSet("paulmin.nl".to_owned(), "876543".to_owned())

        );
    }
}
