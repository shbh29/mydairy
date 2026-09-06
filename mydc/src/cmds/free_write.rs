use crate::cmds::Cmds;
use std::io;
use crate::errors::Errors;
use crate::cmds::write_content::WriteContent;
use crate::models::journal_entry::JournalEntry;
use serde::{Deserialize, Serialize};

pub trait HandleFreeWrite {
    fn handle_free_write(&self) -> Result<bool, Errors>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FreeWrite {
    content: String,
}

impl FreeWrite {
    pub fn new() -> FreeWrite {
        FreeWrite {
            content: String::new(),
        }
    }
    pub fn load() -> FreeWrite {
        FreeWrite::new()
    }
}

impl HandleFreeWrite for Cmds {
    fn handle_free_write(&self) -> Result<bool, Errors> {
        println!("Start writing, and Empty line would stop writing mode");
        let mut je = JournalEntry::load()?;
        let existing_fw = je.free_write();
        println!("Existing: {}", existing_fw.content);
        let mut file_content = String::new();
        let mut input = String::new();
        loop  {
            io::stdin().read_line(&mut input)
                .map_err(|_| Errors::IOError)?;
    
            if input.trim().is_empty() {
                break;
            } else {
                file_content.push_str(&input);
            }
            
            input.clear();
        }

        let mut fw = je.free_write_mut();
        fw.content.push_str(&file_content);

        let je_str = serde_json::to_string(&je)
            .map_err(|_| Errors::InvalidSerializeOp)?;

        Cmds::write(&je_str)
    }
}


