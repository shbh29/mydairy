use crate::cmds::Cmds;
use std::io;
use mydlib::errors::Errors;
use crate::cmds::write_content::WriteContent;
use mydlib::models::journal_entry::JournalEntry;

pub trait HandleFreeWrite {
    fn handle_free_write(&self) -> Result<bool, Errors>;
}

impl HandleFreeWrite for Cmds {
    fn handle_free_write(&self) -> Result<bool, Errors> {
        println!("Start writing, and Empty line would stop writing mode");
        let mut je = JournalEntry::load()?;
        let existing_fw = je.free_write();
        println!("Existing: {}", existing_fw.content());
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

        let fw = je.free_write_mut();
        fw.push_file_content(&file_content);

        let je_str = serde_json::to_string(&je)
            .map_err(|_| Errors::InvalidSerializeOp)?;

        Cmds::write(&je_str)
    }
}


