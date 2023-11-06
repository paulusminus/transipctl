pub(crate) trait StrExtension<'a> {
    fn one_param(self, command: &str) -> Option<&'a str>;
    fn two_params(self, command: &str) -> Option<(&'a str, &'a str)>; 
}

impl<'a> StrExtension<'a> for &'a str {
    fn one_param(self, command: &str) -> Option<&'a str> {
        let command = command.to_owned() + " ";
        if self.starts_with(command.as_str()) {
            let rest = self[command.len()..].trim();
            if rest.find(' ').is_none() {
                Some(rest)
            }
            else {
                None
            }    
        }
        else {
            None
        }
    }

    fn two_params(self, command: &str) -> Option<(&'a str, &'a str)> {
        let command = command.to_owned() + " ";
        if self.starts_with(command.as_str()) {
            let rest = self[command.len()..].trim();
            if let Some(end_first) = rest.find(' ') {
                let param1 = &rest[..end_first];
                if let Some(param2) = rest.one_param(param1) {
                    Some((param1.trim(), param2.trim()))
                }
                else {
                    None
                }
            }
            else {
                None
            }    
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::StrExtension;
    
    #[test]
    fn has_two() {
        assert_eq!(
            "elements  dslkf  lkdjf".two_params("elements"),
            Some(("dslkf", "lkdjf")),
        );

        assert!(
            "elements dkf  dkf dkf".two_params("elements").is_none(),
        );
    }    
}