use std::{fmt::Display, fs::File, path::PathBuf};

use super::*;

#[derive(Debug)]
#[allow(unused)]
pub struct TextFile {
    file: File,
}

impl TextFile {
    #[allow(unused)]
    pub fn new(path: PathBuf) -> Result<TextFile, TextFileError> {
        match File::open(path) {
            Ok(file) => Ok(TextFile { file }),
            Err(e) => Err(TextFileError::Io(e)),
        }
    }

    #[allow(unused)]
    pub fn read(&mut self) -> String {
        todo!()
    }

    #[allow(unused)]
    pub fn write(&mut self, contents: &str) {
        todo!()
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum TextFileError {
    Io(std::io::Error),
    Parse,
    Write(serde_json::Error),
}

impl Display for TextFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl std::error::Error for TextFileError {}

impl DataManager for TextFile {
    fn load(&self) -> Result<Data, impl std::error::Error> {
        Ok::<Data, TextFileError>(Data::default())
    }

    fn save(&mut self, data: Data) -> Result<(), impl std::error::Error> {
        let _ = data;
        Ok::<(), TextFileError>(())
    }
}
