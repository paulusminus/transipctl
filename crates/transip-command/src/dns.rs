use strum::{Display, EnumString};

use crate::{
    check_environment,
    error::{DnsCommandError, TooMany},
    str_extension::Words,
};
use std::str::FromStr;

pub type DomainName = String;

const ACME_VALIDATION_DELETE: &str = "acme-validation-delete";
const ACME_VALIDATION_SET: &str = "acme-validation-set";
#[cfg(feature = "propagation")]
const ACME_VALIDATION_CHECK: &str = "acme-validation-check";
const LIST: &str = "list";
const INSERT: &str = "insert";
const DELETE: &str = "delete";
#[cfg(feature = "propagation")]
const ALL_SUBCOMMANDS: [&str; 6] = [
    ACME_VALIDATION_DELETE,
    ACME_VALIDATION_SET,
    ACME_VALIDATION_CHECK,
    LIST,
    INSERT,
    DELETE,
];
#[cfg(not(feature = "propagation"))]
const ALL_SUBCOMMANDS: [&str; 5] = [
    ACME_VALIDATION_DELETE,
    ACME_VALIDATION_SET,
    LIST,
    INSERT,
    DELETE,
];

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    TXT,
    SRV,
}

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

    Delete(DomainName, String),

    Insert(DomainName, String),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns acme-validation-delete lkdfjf.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
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
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns acme-validation-set lkdfjf.nl oe8rtg";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::AcmeValidationSet(
    ///             "lkdfjf.nl".to_owned(),
    ///             "oe8rtg".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    AcmeValidationSet(DomainName, String),

    #[cfg(feature = "propagation")]
    AcmeValidationCheck(DomainName, String),
}

impl std::fmt::Display for DnsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsCommand::AcmeValidationDelete(name) => {
                write!(f, "{} {}", ACME_VALIDATION_DELETE, name)
            }
            DnsCommand::Delete(name, dns_entry_string) => {
                write!(f, "{} {}", name, dns_entry_string)
            }
            DnsCommand::List(name) => write!(f, "{} {}", LIST, name),
            DnsCommand::Insert(name, dns_entry_string) => {
                write!(f, "{} {}", name, dns_entry_string)
            }
            DnsCommand::AcmeValidationSet(name, challenge) => {
                write!(f, "{} {} {}", ACME_VALIDATION_SET, name, challenge)
            }
            #[cfg(feature = "propagation")]
            DnsCommand::AcmeValidationCheck(name, challenge) => {
                write!(f, "{} {} {}", ACME_VALIDATION_CHECK, name, challenge)
            }
        }
    }
}

impl FromStr for DnsCommand {
    type Err = DnsCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DnsCommand::try_from(Words::from(s))
    }
}

impl<'a> TryFrom<Words<'a>> for DnsCommand {
    type Error = DnsCommandError;
    fn try_from(mut words: Words<'a>) -> Result<Self, Self::Error> {
        let sub_command = words.next().ok_or(DnsCommandError::MissingSubCommand)?;
        if !ALL_SUBCOMMANDS.contains(&sub_command) {
            return Err(DnsCommandError::WrongSubCommand(sub_command.to_owned()));
        }

        let domain_name = words.next().ok_or(DnsCommandError::DomainNameMissing)?;

        if sub_command == LIST {
            words
                .next()
                .too_many()
                .map_err(DnsCommandError::TooManyParameters)?;
            return Ok(DnsCommand::List(check_environment(domain_name)?));
        }

        if sub_command == ACME_VALIDATION_DELETE {
            words
                .next()
                .too_many()
                .map_err(DnsCommandError::TooManyParameters)?;
            return Ok(DnsCommand::AcmeValidationDelete(check_environment(
                domain_name,
            )?));
        }

        #[cfg(feature = "propagation")]
        if sub_command == ACME_VALIDATION_CHECK {
            {
                let validation = words.next().ok_or(DnsCommandError::ValidationMissing)?;
                words
                    .next()
                    .too_many()
                    .map_err(DnsCommandError::TooManyParameters)?;
                return Ok(DnsCommand::AcmeValidationCheck(
                    check_environment(domain_name)?,
                    check_environment(validation)?,
                ));
            }
        }

        if sub_command == ACME_VALIDATION_SET {
            let validation = words.next().ok_or(DnsCommandError::ValidationMissing)?;
            return words
                .next()
                .too_many()
                .map_err(DnsCommandError::TooManyParameters)
                .and_then(|_| check_environment(domain_name).map_err(DnsCommandError::Environment))
                .and_then(|domain| {
                    check_environment(validation)
                        .map_err(DnsCommandError::Environment)
                        .map(|validation| DnsCommand::AcmeValidationSet(domain, validation))
                });
        }

        if sub_command == DELETE {
            let dns_entry = dns_entry_string(words)?;
            return Ok(DnsCommand::Delete(
                check_environment(domain_name)?,
                dns_entry,
            ));
        }

        if sub_command == INSERT {
            let dns_entry = dns_entry_string(words)?;
            return Ok(DnsCommand::Insert(
                check_environment(domain_name)?,
                dns_entry,
            ));
        }
        Err(DnsCommandError::MissingSubCommand)
    }
}

fn dns_entry_string(mut words: Words<'_>) -> Result<String, DnsCommandError> {
    let dns_name = words.next().ok_or(DnsCommandError::NoDnsRecordName)?;
    let ttl = words
        .next()
        .ok_or(DnsCommandError::NoTTL)
        .and_then(|s| s.parse::<u64>().map_err(DnsCommandError::InvalidTTL))?;
    let record_type = words.next().ok_or(DnsCommandError::RecordTypeMissing)?;

    let record_type = record_type
        .parse::<RecordType>()
        .map_err(DnsCommandError::InvalidRecordType)?;
    if let Some(content) = words.rest() {
        Ok(format!(
            "{} {} {} {}",
            dns_name,
            ttl,
            record_type,
            check_environment(content)?,
        ))
    } else {
        Err(DnsCommandError::ContentMissing)
    }
}

#[cfg(test)]
mod test {
    use crate::str_extension::Words;

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
            DnsCommand::try_from(Words::from(
                "acme-validation-set ${CERTBOT_DOMAIN} ${CERTBOT_VALIDATION}"
            ))
            .unwrap(),
            DnsCommand::AcmeValidationSet("paulmin.nl".to_owned(), "876543".to_owned())
        );
    }
}
