use crate::cmds::Cmds;
use std::io;
use crate::errors::Errors;
use crate::cmds::write_content::WriteContent;

pub trait FreeWrite {
    fn handle_free_write(&self) -> Result<bool, Errors>;
}

// TODO: Add generics in the definition
// TODO: 
impl FreeWrite for Cmds {
    fn handle_free_write(&self) -> Result<bool, Errors> {
        println!("Start writing, and Empty line would stop writing mode");

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

        Cmds::write(&file_content)
    }
}


