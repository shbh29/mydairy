use crate::errors::Errors;
use crate::cmds::Cmds;
use crate::models::check_list::CheckList;
use serde::{Deserialize, Serialize};

pub trait HandleIntentions {
     /*I don't actually need the intent parameter but, intent helps separate the CRUD methods from
      * Cmds enum. So, I created and passed an empty intent. I will populate this intent based on
      * the current.json content.*/
    fn handle_intentions() -> Result<bool, Errors>;
}

pub trait Intentions {
    fn add(&mut self) -> bool;
    fn remove(&mut self) -> bool;
    fn mark_done(&mut self) -> bool;
    fn mark_undone(&mut self) -> bool;
    fn move_to_cf(&mut self) -> bool;
}

impl HandleIntentions for Cmds {
    fn handle_intentions() -> Result<bool, Errors> {
        println!("Intentions: ");
        Ok(true)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Intent {
    list: Vec<CheckList>
}

impl Intent {
    pub fn new() -> Intent {
        Intent {
            list: vec![],
        }
    }
    pub fn load() -> Intent {
        // Load the intent from the file.
        Intent::new()
    }
}
    
impl Intentions for Intent {
    fn add(&mut self) -> bool { true }
    fn remove(&mut self) -> bool { true }
    fn mark_done(&mut self) -> bool { true }
    fn mark_undone(&mut self) -> bool { true }
    fn move_to_cf(&mut self) -> bool { true }
}


