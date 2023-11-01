use std::{
    env::Args,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read},
};

use crate::{error::Error, Result};

pub struct Input {
    pub reader: Box<dyn Read>,
}

impl Input {
    pub fn lines(self) -> impl Iterator<Item = String> {
        BufReader::new(self.reader).lines().flatten()
    }
}

impl TryFrom<Args> for Input {
    type Error = Error;
    fn try_from(mut args: Args) -> Result<Self> {
        if let Some(file_name) = args.nth(1) {
            let file = OpenOptions::new().read(true).open(file_name)?;
            Ok(Self {
                reader: Box::new(file),
            })
        } else {
            Ok(Self {
                reader: Box::new(std::io::stdin()),
            })
        }
    }
}
