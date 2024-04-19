use regex::{Captures, Regex};
use rustyline::error::ReadlineError;

use crate::{Error, Result};
use std::{
    env::{var, VarError},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Cursor, Lines, Read},
    path::Path,
};

const CAPTURE: &str = r#"\$\{([A-Z][A-Z_]*)}"#;

fn regex() -> Regex {
    Regex::new(CAPTURE).unwrap()
}

pub struct FileReader<R: Read> {
    lines: Lines<BufReader<R>>,
    re: Regex,
    replace_variables: bool,
}

impl<R: Read> FileReader<R> {
    pub fn try_new<P: AsRef<Path>>(path: P, replace_variables: bool) -> Result<FileReader<File>> {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(ReadlineError::from)
            .map_err(Error)
            .map(BufReader::new)
            .map(|reader| reader.lines())
            .map(|lines| FileReader::<File> {
                lines,
                re: regex(),
                replace_variables,
            })
    }
}

impl From<String> for FileReader<Cursor<String>> {
    fn from(s: String) -> FileReader<Cursor<String>> {
        let lines = BufReader::new(Cursor::new(s)).lines();
        FileReader {
            lines,
            re: regex(),
            replace_variables: true,
        }
    }
}

impl<R: Read> Iterator for FileReader<R> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|result| result.map_err(ReadlineError::from).map_err(Error))
            .map(|result| {
                result.map(|s| {
                    if self.replace_variables {
                        replace_enviroment_variables(s, &self.re)
                    } else {
                        s
                    }
                })
            })
    }
}

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

fn replace_enviroment_variables(haystack: String, re: &Regex) -> String {
    let replacement = |caps: &Captures| -> Result<String, VarError> {
        var(caps.get(1).unwrap().as_str()).map(|s| format!("\"{s}\""))
    };
    replace_all(re, &haystack, replacement).unwrap()
}

#[cfg(test)]
mod test {
    use super::{regex, replace_all, FileReader};
    use regex::Captures;
    use std::env::{set_var, var, VarError};

    #[test]
    fn variable_substition() {
        set_var("CERTBOT_DOMAIN", "GOOGLE.COM");
        set_var("CERTBOT_VALIDATION", "lksjfoie9");

        let haystack =
            "dns acme-validation-set ${CERTBOT_DOMAIN}   ${CERTBOT_VALIDATION}".to_owned();
        let replacement =
            |caps: &Captures| -> Result<String, VarError> { var(caps.get(1).unwrap().as_str()) };
        let new = replace_all(&regex(), &haystack, &replacement).unwrap();
        assert_eq!(new, *"dns acme-validation-set GOOGLE.COM   lksjfoie9");
    }

    #[test]
    fn variable_substition_in_iterator() {
        set_var("CERTBOT_DOMAIN", "GOOGLE.COM");
        set_var("CERTBOT_VALIDATION", "lksjfoie9");
        set_var("DOMAIN", "paulmin.nl");

        let lines =
            "dns acme-validation-set ${CERTBOT_DOMAIN}   ${CERTBOT_VALIDATION}\ndns list ${DOMAIN}"
                .to_owned();
        let f = FileReader::from(lines);

        let result = f.collect::<crate::Result<Vec<_>>>().unwrap();
        assert_eq!(
            result[0],
            *"dns acme-validation-set \"GOOGLE.COM\"   \"lksjfoie9\""
        );
        assert_eq!(result[1], *"dns list \"paulmin.nl\"");
    }
}
