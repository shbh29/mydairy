use serde::{Deserialize, Serialize};
use crate::models::check_list::CheckList;

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
    pub fn display(&self) {
        self.list.iter().enumerate()
            .for_each(|(index, check_list_item)| {
                let is_done: char = match check_list_item.is_done() { 
                    true => 'x',
                    false => ' ',
                };
                println!(" {}. [{}] {}", index+1, is_done, check_list_item.task());
            });
    }
    pub fn add(&mut self, task: String) -> bool { 
        let check_list = CheckList::with_task(task);
        self.list.push(check_list);
        true
    }
    pub fn remove(&mut self, index: usize) -> bool { self.list.remove(index); true }
    pub fn toggle_done(&mut self, index: usize) -> bool { self.list[index].toggle_done() }
    pub fn move_to_cf(&mut self, _index: usize) -> bool { true }
}


