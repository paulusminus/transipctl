pub(crate) trait StrExtension<'a> {
    fn one_param(self, command: &str) -> Option<&'a str>;
    fn two_params(self, command: &str) -> Option<(&'a str, &'a str)>;
}

impl<'a> StrExtension<'a> for &'a str {
    fn one_param(self, command: &str) -> Option<&'a str> {
        self.strip_prefix((command.to_owned() + " ").as_str())
            .map(|s| s.trim())
            .and_then(|rest| {
                if rest.find(' ').is_none() {
                    Some(rest)
                } else {
                    None
                }
            })
    }

    fn two_params(self, command: &str) -> Option<(&'a str, &'a str)> {
        self.strip_prefix((command.to_owned() + " ").as_str())
            .map(|s| s.trim())
            .and_then(|rest| {
                if let Some(end_first) = rest.find(' ') {
                    let param1 = &rest[..end_first];
                    rest.one_param(param1).map(|param2| (param1, param2.trim()))
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod test {
    use super::StrExtension;

    #[test]
    fn has_two() {
        assert_eq!(
            "elements   dslkf    lkdjf  ".two_params("elements"),
            Some(("dslkf", "lkdjf")),
        );

        assert!("elements dkf  dkf dkf".two_params("elements").is_none(),);
    }
}