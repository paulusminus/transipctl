use regex::{Captures, Regex};

use crate::Result;
use std::{
    env::{var, VarError},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Lines},
    path::Path,
};

pub struct FileReader {
    lines: Lines<BufReader<File>>,
    re: Regex,
    replace_variables: bool,
}

impl FileReader {
    pub fn try_new<P: AsRef<Path>>(path: P, replace_variables: bool) -> Result<Self> {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(Into::into)
            .map(BufReader::new)
            .map(|reader| reader.lines())
            .map(|lines| Self {
                lines,
                re: Regex::new(r#"\$\{([A-Z][A-Z_]*)}"#).unwrap(),
                replace_variables,
            })
    }
}

impl Iterator for FileReader {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|result| result.map_err(Into::into))
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
    use super::replace_all;
    use regex::{Captures, Regex};
    use std::env::{var, VarError};

    #[test]
    fn test() {
        let re = Regex::new(r#"\$\{([A-Z][A-Z_]*)}"#).unwrap();

        let haystack =
            "dns acme-validation-set ${CERTBOT_DOMAIN}   ${CERTBOT_VALIDATION}".to_owned();
        let replacement =
            |caps: &Captures| -> Result<String, VarError> { var(caps.get(1).unwrap().as_str()) };
        let new = replace_all(&re, &haystack, &replacement).unwrap();
        println!("{}", new);
    }
}
