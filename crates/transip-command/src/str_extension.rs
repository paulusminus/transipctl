pub struct Words<'a>(&'a str);

impl<'a> From<&'a str> for Words<'a> {
    fn from(s: &'a str) -> Self {
        Words(s)
    }
}

impl<'a> Words<'a> {
    pub fn rest(&self) -> &'a str {
        self.0
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.trim().is_empty() {
            None
        } else {
            let s = self.0.trim_start();
            if let Some(index) = s.find(' ') {
                self.0 = s[index..].trim();
                Some(&s[..index])
            } else {
                self.0 = "";
                Some(s)
            }
        }
    }
}

pub(crate) trait StrExtension<'a> {
    fn one_param(self, command: &str) -> Option<&'a str>;
    fn two_params(self, command: &str) -> Option<(&'a str, &'a str)>;
}

impl<'a> StrExtension<'a> for &'a str {
    fn one_param(self, command: &str) -> Option<&'a str> {
        self.strip_prefix((command.to_owned() + " ").as_str())
            .map(|s| s.trim())
            .and_then(|rest| {
                if !rest.is_empty() && rest.trim().find(' ').is_none() {
                    Some(rest.trim())
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
    use super::{StrExtension, Words};

    #[test]
    fn words() {
        let mut words = Words::from("  Dit is een   test om te kijken of het  werkt");
        assert_eq!(
            words.rest(),
            "  Dit is een   test om te kijken of het  werkt"
        );
        assert_eq!(words.next(), Some("Dit"));
        assert_eq!(words.rest(), "is een   test om te kijken of het  werkt");
        assert_eq!(words.next(), Some("is"));
        assert_eq!(words.rest(), "een   test om te kijken of het  werkt");
        assert_eq!(words.next(), Some("een"));
        assert_eq!(words.rest(), "test om te kijken of het  werkt");
        assert_eq!(words.next(), Some("test"));
        assert_eq!(words.rest(), "om te kijken of het  werkt");
        assert_eq!(words.next(), Some("om"));
        assert_eq!(words.rest(), "te kijken of het  werkt");
        assert_eq!(words.next(), Some("te"));
        assert_eq!(words.rest(), "kijken of het  werkt");
        assert_eq!(words.next(), Some("kijken"));
        assert_eq!(words.rest(), "of het  werkt");
        assert_eq!(words.next(), Some("of"));
        assert_eq!(words.rest(), "het  werkt");
        assert_eq!(words.next(), Some("het"));
        assert_eq!(words.rest(), "werkt");
        assert_eq!(words.next(), Some("werkt"));
        dbg!(words.rest(), "");
        assert_eq!(words.next(), None);
        dbg!(words.rest());
    }

    #[test]
    fn has_one() {
        assert_eq!("elements  dlkf".one_param("elements"), Some("dlkf"),);

        assert!("elements dlkjf kgjf".one_param("elements").is_none());
        assert_eq!("elements   ".one_param("elements"), None);
    }

    #[test]
    fn has_two() {
        assert_eq!(
            "elements   dslkf    lkdjf  ".two_params("elements"),
            Some(("dslkf", "lkdjf")),
        );

        assert!("elements dkf  dkf dkf".two_params("elements").is_none(),);
        assert!("elements dkf  ".two_params("elements").is_none());
        assert!("elements  ".two_params("elements").is_none());
    }

    #[test]
    fn test_split() {
        let s = "dit is een  mooi dag   om werkelijk waar te zijn";
        let mut splitted = s.split_ascii_whitespace();

        assert_eq!(splitted.next(), Some("dit"),);

        assert_eq!(splitted.next(), Some("is"),);
        assert_eq!(
            splitted.collect::<Vec<_>>(),
            vec![
                "een",
                "mooi",
                "dag",
                "om",
                "werkelijk",
                "waar",
                "te",
                "zijn"
            ],
        );
    }
}
