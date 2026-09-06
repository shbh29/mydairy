use crate::errors::Errors;
use crate::cmds::Cmds;
use crate::models::check_list::CheckList;
use serde::{Deserialize, Serialize};
use crate::models::journal_entry::JournalEntry;
use crate::cmds::write_content::WriteContent;

pub trait HandleIntentions {
    fn handle_intentions(inpts: Vec<String>) -> Result<bool, Errors>;
}

impl HandleIntentions for Cmds {
    fn handle_intentions(inpts: Vec<String>) -> Result<bool, Errors> {
        let mut je = JournalEntry::load()?;
        println!("Intentions: ");

        let intent: &mut Intentions = je.intentions_mut();

        let sub_cmd = inpts.get(1)
            .map(|s| s.as_str());

        let _ = match sub_cmd {
            Some("add") => intent.add(inpts[2..].join(" ")),
            Some("rm") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                // this can throw invalid index exception
                intent.remove(index - 1)
            },
            Some("dn") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                intent.toggle_done(index - 1)
            },
            Some("mvcf") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                intent.move_to_cf(index - 1)
            },
            None | _ => {
                return Err(Errors::InvalidCmd);
            }
        };

        intent.display();
        
        let je_str = serde_json::to_string(&je)
            .map_err(|_| Errors::InvalidSerializeOp)?;

        Cmds::write(&je_str)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Intentions {
    list: Vec<CheckList>
}

impl Intentions {
    pub fn new() -> Intentions {
        Intentions {
            list: vec![],
        }
    }
}
    
impl Intentions {
    fn display(&self) {
        self.list.iter().enumerate()
            .for_each(|(index, check_list_item)| {
                let is_done: char = match check_list_item.is_done() { 
                    true => 'x',
                    false => ' ',
                };
                println!(" {}. [{}] {}", index+1, is_done, check_list_item.task());
            });
    }
    fn add(&mut self, task: String) -> bool { 
        let check_list = CheckList::with_task(task);
        self.list.push(check_list);
        true
    }
    fn remove(&mut self, index: usize) -> bool { self.list.remove(index); true }
    fn toggle_done(&mut self, index: usize) -> bool { self.list[index].toggle_done() }
    fn move_to_cf(&mut self, index: usize) -> bool { true }
}


