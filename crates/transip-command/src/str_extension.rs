pub struct Words<'a>(&'a str);

impl<'a> From<&'a str> for Words<'a> {
    fn from(s: &'a str) -> Self {
        Words(s)
    }
}

impl<'a> Words<'a> {
    pub fn rest(&mut self) -> Option<&'a str> {
        let rest = self.0.trim();
        self.0 = "";
        if rest.is_empty() {
            None
        } else {
            Some(rest)
        }
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

// pub(crate) trait StrExtension<'a> {
//     fn one_param(self, command: &str) -> Option<&'a str>;
//     fn two_params(self, command: &str) -> Option<(&'a str, &'a str)>;
// }

// impl<'a> StrExtension<'a> for &'a str {
//     fn one_param(self, command: &str) -> Option<&'a str> {
//         self.strip_prefix((command.to_owned() + " ").as_str())
//             .map(|s| s.trim())
//             .and_then(|rest| {
//                 if !rest.is_empty() && rest.trim().find(' ').is_none() {
//                     Some(rest.trim())
//                 } else {
//                     None
//                 }
//             })
//     }

//     fn two_params(self, command: &str) -> Option<(&'a str, &'a str)> {
//         self.strip_prefix((command.to_owned() + " ").as_str())
//             .map(|s| s.trim())
//             .and_then(|rest| {
//                 if let Some(end_first) = rest.find(' ') {
//                     let param1 = &rest[..end_first];
//                     rest.one_param(param1).map(|param2| (param1, param2.trim()))
//                 } else {
//                     None
//                 }
//             })
//     }
// }

#[cfg(test)]
mod test {
    use super::Words;

    fn get_words() -> Words<'static> {
        Words::from("  Dit is een   test om te kijken of het  werkt   ")
    }

    #[test]
    fn words() {
        assert_eq!(
            get_words().rest(),
            Some("Dit is een   test om te kijken of het  werkt"),
        );

        {
            let mut words = get_words();
            assert_eq!(words.next(), Some("Dit"));
            assert_eq!(
                words.rest(),
                Some("is een   test om te kijken of het  werkt")
            );
        }

        {
            let mut words = get_words();
            assert_eq!(words.next(), Some("Dit"));
            assert_eq!(words.next(), Some("is"));
            assert_eq!(words.rest(), Some("een   test om te kijken of het  werkt"));
        }

        {
            let mut words = get_words();
            assert_eq!(words.next(), Some("Dit"));
            assert_eq!(words.next(), Some("is"));
            assert_eq!(words.next(), Some("een"));
            assert_eq!(words.rest(), Some("test om te kijken of het  werkt"));
        }
    }

    // #[test]
    // fn has_one() {
    //     assert_eq!("elements  dlkf".one_param("elements"), Some("dlkf"),);

    //     assert!("elements dlkjf kgjf".one_param("elements").is_none());
    //     assert_eq!("elements   ".one_param("elements"), None);
    // }

    // #[test]
    // fn has_two() {
    //     assert_eq!(
    //         "elements   dslkf    lkdjf  ".two_params("elements"),
    //         Some(("dslkf", "lkdjf")),
    //     );

    //     assert!("elements dkf  dkf dkf".two_params("elements").is_none(),);
    //     assert!("elements dkf  ".two_params("elements").is_none());
    //     assert!("elements  ".two_params("elements").is_none());
    // }

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
