pub mod text_file;

use crate::data::Data;
use std::error::Error;

pub trait DataManager {
    fn load(&self) -> Result<Data, impl Error>;
    fn save(&mut self, data: Data) -> Result<(), impl Error>;
}
