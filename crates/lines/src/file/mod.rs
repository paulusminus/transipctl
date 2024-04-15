use crate::Result;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Lines},
    path::Path,
};

pub struct FileReader(Lines<BufReader<File>>);

impl FileReader {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self> {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(Into::into)
            .map(BufReader::new)
            .map(|reader| reader.lines())
            .map(Self)
    }
}

impl Iterator for FileReader {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| r.map_err(Into::into))
    }
}
