use crate::{error::Error, Result};
use std::{
    env::Args,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

pub struct Input {
    reader: Box<dyn Read>,
    script: Option<String>,
}

impl Input {
    pub fn run_from(&self) -> (bool, String) {
        self.script
            .as_ref()
            .map(|script| (false, format!("script {}", &script)))
            .unwrap_or((true, "interactive".into()))
    }

    pub fn lines(self) -> impl Iterator<Item = String> {
        BufReader::new(self.reader)
            .lines()
            .map_while(std::io::Result::ok)
    }
}

impl TryFrom<Args> for Input {
    type Error = Error;
    fn try_from(mut args: Args) -> Result<Self> {
        match args.nth(1) {
            Some(file_name) => File::open(&file_name).map_err(Into::into).map(|file| Self {
                reader: Box::new(file),
                script: Some(file_name),
            }),
            None => Ok(Self {
                reader: Box::new(stdin()),
                script: None,
            }),
        }
    }
}
