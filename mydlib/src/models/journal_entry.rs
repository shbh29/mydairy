use chrono::{NaiveDate, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use crate::errors::Errors;
use std::io::Read;
use crate::models::free_write::FreeWrite;
use crate::models::intentions::Intentions;





#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    id: String,
    date: NaiveDate,
    intentions: Intentions,
    free_write: FreeWrite,
}

impl JournalEntry {
    pub fn new() -> JournalEntry {
        JournalEntry {
            id: JournalEntry::get_id_string(),
            date: Local::now().date_naive(),
            intentions: Intentions::new(),
            free_write: FreeWrite::new(),
        }
    }
    pub fn load() -> Result<JournalEntry, Errors> {

        //file to str
        let current_file = File::open("current.json")
            .map_err(|_| Errors::UnableToOpenFile)?;
        let mut je_str = String::new();
        let mut je_str_br = BufReader::new(current_file);
        je_str_br.read_to_string(&mut je_str) 
            .map_err(Errors::InvalidDeserializeOp)?;

        //str to json
        let je: JournalEntry = serde_json::from_str(&je_str) 
            .map_err(Errors::InvalidJsonInFile)?;
        
        Ok(je)
    }
    fn get_id_string() -> String {
        let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        (0..10)
            .map(|_| chars[rand::random_range(0..chars.len())] as char)
            .collect()
    }
    pub fn free_write(&self) -> &FreeWrite {
        &self.free_write
    }
    pub fn free_write_mut(&mut self) -> &mut FreeWrite {
        &mut self.free_write
    }
    pub fn intentions(&self) -> &Intentions {
        &self.intentions
    }
    pub fn intentions_mut(&mut self) -> &mut Intentions {
        &mut self.intentions
    }

}
