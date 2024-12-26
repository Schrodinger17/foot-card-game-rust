use crate::data::Data;
use crate::data_manager::text_file::TextFile;
use crate::data_manager::DataManager;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Server {
    data: Data,
}

impl Server {
    pub fn start(&self) {
        println!("Server started !");
    }

    pub fn init(&mut self) {
        let data_path = PathBuf::from("data/playerList.txt".to_owned());
        let data_manager = TextFile::new(data_path);

        self.data = match data_manager {
            Ok(data_manager) => {
                println!("Data manager created !");
                match data_manager.load() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        Data::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Data::default()
            }
        };
    }
}
