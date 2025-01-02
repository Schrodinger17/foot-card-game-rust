use std::{fmt::Display, fs::File, path::PathBuf};

use super::*;

#[derive(Debug)]
pub struct TextFile {
    file: File,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Result<TextFile, TextFileError> {
        match File::open(path) {
            Ok(file) => Ok(TextFile { file }),
            Err(e) => Err(TextFileError::Io(e)),
        }
    }

    pub fn read(&mut self) -> String {
        todo!()
    }

    pub fn write(&mut self, contents: &str) {
        todo!()
    }
}

#[derive(Debug)]
pub enum TextFileError {
    Io(std::io::Error),
    Parse,
    Write(serde_json::Error),
}

impl Display for TextFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for TextFileError {}

impl DataManager for TextFile {
    fn load(&self) -> Result<Data, TextFileError> {
        todo!()
    }

    fn save(&mut self, data: Data) -> Result<(), TextFileError> {
        todo!()
    }
}
