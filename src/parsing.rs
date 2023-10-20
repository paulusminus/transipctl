fn split_in_first_rest(s: &str) -> Option<(&str, &str)> {
    let whatever = s.trim();
    if let Some(i) = whatever.find(' ') {
        Some((&s[..i], &s[i..].trim()))
    }
    else {
        None
    }
}

struct VpsName(String);

struct VpsDescription {
    name: VpsName,
    description: String,
}

struct VpsLocked {
    name: VpsName,
    locked: bool,
}

enum VpsCommand {
    List,
    Entry(VpsName),
    Start(VpsName),
    Stop(VpsName),
    Reset(VpsName),
    Description(VpsDescription),
    SetLocked(String, bool),
}

impl FromStr for VpsCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.trim();
        if command.to_lowercase() == "list" {
            return Ok(VpsCommand::List)
        }
        else if command.to_lowercase().starts_with("entry ") {
            if command[6..].split_ascii_whitespace().count() == 1 {
                return Ok(VpsCommand::Entry(s[6..].trim().to_owned()));
            }
        }
        else if s.to_lowercase().trim().starts_with("start ") {
            let name = &s[6..];
            return Ok(VpsCommand::Start(name.trim().to_owned()));
        }
        else if s.to_lowercase().trim().starts_with("stop ") {
            let name = &s[5..];
            return Ok(VpsCommand::Stop(name.trim().to_owned()));
        }
        else if s.to_lowercase().trim().starts_with("reset ") {
            let name = &s[5..];
            return Ok(VpsCommand::Stop(name.trim().to_owned()));
        }
        else {
            return Err(Error::ParseProductCommand(s.to_owned()));
        }
    }
}

enum DnsCommand {
    List,
    Entry(String),
}


#[derive(Debug, PartialEq)]
enum ProductCommand {
    List,
    Elements(String),
}

impl FromStr for ProductCommand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wrong = Err(Error::ParseProductCommand(s.to_owned()));
        let iter: Vec<&str> = s.split_ascii_whitespace().collect();
        if iter.len() == 0 { return wrong; }
        if iter[0].to_lowercase() == "list" && iter.len() == 1 { 
            return Ok(ProductCommand::List); 
        }
        if iter[0].to_lowercase() == "elements" && iter.len() == 2 { 
            return Ok(ProductCommand::Elements(iter[1].to_owned())); 
        }
        return wrong;
    }
}


enum Command {
    Vps(VpsCommand),
    Dns(DnsCommand),
    Product(ProductCommand),
}

#[cfg(test)]
mod test {
    use crate::ProductCommand;
    use super::split_in_first_rest;

    #[test]
    fn product_command_list_ok() {
        let result = "list".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::List);

        let result = "list ".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::List);

        let result = "   list ".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::List);
    }

    #[test]
    fn product_command_elements_ok() {
        let result = "elements vps".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::Elements("vps".to_owned()));

        let result = " elements kubernetes".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::Elements("kubernetes".to_owned()));

        let result = "elements  key".parse::<ProductCommand>().unwrap();
        assert_eq!(result, ProductCommand::Elements("key".to_owned()));
    }


    #[test]
    fn find_first_space() {
        let (first, rest) = split_in_first_rest("list testen maar").unwrap();
        assert_eq!(first, "list");
        assert_eq!(rest, "testen maar");

    }
}