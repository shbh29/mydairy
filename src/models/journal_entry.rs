use rand::Rng;
use chrono::{NaiveDate, Local};
use crate::cmds::intentions::Intent;
use crate::cmds::free_write::FreeWrite;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    id: String,
    date: NaiveDate,
    intentions: Intent,
    free_write: FreeWrite,
}

impl JournalEntry {
    pub fn new() -> JournalEntry {
        JournalEntry {
            id: JournalEntry::get_id_string(),
            date: Local::now().date_naive(),
            intentions: Intent::new(),
            free_write: FreeWrite::new(),
        }
    }
    pub fn load() -> JournalEntry {
        JournalEntry::new()
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
}
