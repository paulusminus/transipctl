use crate::{error::Error, Result};
use std::{
    env::Args,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read},
};

pub struct Input {
    reader: Box<dyn Read>,
    script: Option<String>,
}

impl Input {
    pub fn run_from(&self) -> String {
        match &self.script {
            Some(script) => format!("from script {}", script),
            None => "interactively".into(),
        }
    }

    pub fn lines(self) -> impl Iterator<Item = String> {
        BufReader::new(self.reader).lines().flatten()
    }
}

impl TryFrom<Args> for Input {
    type Error = Error;
    fn try_from(mut args: Args) -> Result<Self> {
        if let Some(file_name) = args.nth(1) {
            let file = OpenOptions::new().read(true).open(&file_name)?;
            Ok(Self {
                reader: Box::new(file),
                script: Some(file_name),
            })
        } else {
            Ok(Self {
                reader: Box::new(std::io::stdin()),
                script: None,
            })
        }
    }
}
