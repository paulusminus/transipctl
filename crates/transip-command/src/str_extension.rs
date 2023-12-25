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
}
