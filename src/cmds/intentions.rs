use crate::errors::Errors;
use crate::cmds::Cmds;
use crate::models::CheckList;

pub trait HandleIntentions {
    fn handle_intentions(&self, _intent: Intent) -> Result<bool, Errors>;
}

pub trait Intentions {
    fn add(&mut self) -> bool;
    fn remove(&mut self) -> bool;
    fn mark_done(&mut self) -> bool;
    fn mark_undone(&mut self) -> bool;
    fn move_to_cf(&mut self) -> bool;
}

impl HandleIntentions for Cmds {
    fn handle_intentions(&self, intent: Intent) -> Result<bool, Errors> {
        println!("Intentions: ");
        Ok(true)
    }
}

#[derive(Clone)]
pub struct Intent {
    list: Vec<CheckList>
}

impl Intent {
    pub fn new() -> Intent {
        Intent {
            list: vec![],
        }
    }
}
    
impl Intentions for Intent {
    fn add(&mut self) -> bool { true }
    fn remove(&mut self) -> bool { true }
    fn mark_done(&mut self) -> bool { true }
    fn mark_undone(&mut self) -> bool { true }
    fn move_to_cf(&mut self) -> bool { true }
}


