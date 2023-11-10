use crate::{check_environment, error::Error, str_extension::StrExtension};
use std::{
    fmt::Display,
    str::{FromStr, SplitAsciiWhitespace},
};

pub type DomainName = String;

const ACME_VALIDATION_DELETE: &str = "acme-validation-delete";
const ACME_VALIDATION_SET: &str = "acme-validation-set";
#[cfg(feature = "propagation")]
const ACME_VALIDATION_CHECK: &str = "acme-validation-check";
const LIST: &str = "list";
const INSERT: &str = "insert";
const DELETE: &str = "delete";
const RECORD_TYPES: [&str; 6] = ["A", "AAAA", "CNAME", "MX", "TXT", "SRV"];

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

impl Display for DnsCommand {
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
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(domain_name) = s.one_param(ACME_VALIDATION_DELETE) {
            return Ok(DnsCommand::AcmeValidationDelete(check_environment(
                domain_name,
            )?));
        }
        if let Some((domain_name, challenge)) = s.two_params(ACME_VALIDATION_SET) {
            return Ok(DnsCommand::AcmeValidationSet(
                check_environment(domain_name)?,
                check_environment(challenge)?,
            ));
        }
        #[cfg(feature = "propagation")]
        if let Some((domain_name, challenge)) = s.two_params(ACME_VALIDATION_CHECK) {
            return Ok(DnsCommand::AcmeValidationCheck(
                check_environment(domain_name)?,
                check_environment(challenge)?,
            ));
        }
        if let Some(domain_name) = s.one_param(LIST) {
            return Ok(DnsCommand::List(check_environment(domain_name)?));
        }

        let mut splitted = s.split_ascii_whitespace();
        let command = splitted.next();
        if command == Some(DELETE) {
            let domain_name = splitted.next().ok_or(Error::ParseDnsCommand(
                "domain name not provided".to_owned(),
            ))?;
            let dns_entry = dns_entry_string(splitted)?;
            return Ok(DnsCommand::Delete(
                check_environment(domain_name)?,
                dns_entry,
            ));
        }

        if command == Some(INSERT) {
            let domain_name = splitted.next().ok_or(Error::ParseDnsCommand(
                "domain name not provided".to_owned(),
            ))?;
            let dns_entry = dns_entry_string(splitted)?;
            return Ok(DnsCommand::Insert(
                check_environment(domain_name)?,
                dns_entry,
            ));
        }

        Err(Error::ParseDnsCommand(s.to_owned()))
    }
}

fn dns_entry_string(mut splitted: SplitAsciiWhitespace) -> Result<String, Error> {
    let dns_name = splitted
        .next()
        .ok_or(Error::ParseDnsCommand("dns name not provided".to_owned()))?;
    let ttl = splitted
        .next()
        .ok_or(Error::ParseDnsCommand("ttl not provided".to_owned()))
        .and_then(|s| {
            s.parse::<u64>()
                .map_err(|_| Error::ParseDnsCommand("ttl sould be a postive number".to_owned()))
        })?;
    let record_type = splitted.next().ok_or(Error::ParseDnsCommand(
        "record type name not provided".to_owned(),
    ))?;
    if !RECORD_TYPES.contains(&record_type) {
        return Err(Error::ParseDnsCommand(format!(
            "record type must be one of {}",
            RECORD_TYPES.join(" ")
        )));
    }
    let content = splitted.collect::<Vec<_>>().join(" ");
    if content.is_empty() {
        return Err(Error::ParseDnsCommand("content not provided".to_owned()));
    }
    Ok(format!(
        "{} {} {} {}",
        dns_name,
        ttl,
        record_type,
        check_environment(&content)?
    ))
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
            "acme-validation-set ${CERTBOT_DOMAIN} ${CERTBOT_VALIDATION}"
                .parse::<DnsCommand>()
                .unwrap(),
            DnsCommand::AcmeValidationSet("paulmin.nl".to_owned(), "876543".to_owned())
        );
    }
}
