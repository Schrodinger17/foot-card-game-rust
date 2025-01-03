use std::{fs::File, path::PathBuf};

use crate::data::Data;

use super::{text_file::TextFileError, DataManager};

#[derive(Debug)]
pub struct Json {
    path: PathBuf,
}

impl Json {
    pub fn new(path: PathBuf) -> Json {
        Json { path }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(unused)]
struct JsonError;

impl DataManager for Json {
    fn load(&self) -> Result<Data, impl std::error::Error> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(TextFileError::Io(e)),
        };
        match serde_json::from_reader(&file) {
            Ok(data) => Ok(data),
            Err(_) => Err(TextFileError::Parse),
        }
    }

    fn save(&mut self, data: Data) -> Result<(), impl std::error::Error> {
        let file = match File::create(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(TextFileError::Io(e)),
        };
        match serde_json::to_writer(&file, &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(TextFileError::Write(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_json() {
        let data = Data::default();
        let data_copy = data.clone();
        let serialized = serde_json::to_string(&data).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);

        assert!(data == data_copy)
    }

    #[test]
    fn test_json_save() {
        let path = PathBuf::from("data/test/test_save_default.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::File::create(&path).unwrap();
        let data = Data::default();
        let mut json = Json::new(path);
        json.save(data).unwrap();
    }

    #[test]
    fn test_json_load() {
        let path = PathBuf::from("data/test/test_load_default.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::File::create(&path).unwrap();
        let data = Data::default();
        let mut json = Json::new(path.clone());
        json.save(data.clone()).unwrap();
        let loaded_data = json.load().unwrap();
        assert_eq!(loaded_data, data);
    }
}
