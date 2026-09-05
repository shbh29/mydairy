use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CheckList {
    task: String,
    is_done: bool,
}

impl CheckList {
    pub fn new() -> CheckList {
        CheckList {
            task: String::new(),
            is_done: false,
        }
    }
    pub fn with_task(task: String) -> CheckList {
        CheckList { task, is_done: false }
    }
    pub fn is_done(&self) -> bool {
       self.is_done
    }
    pub fn toggle_done(&mut self) -> bool {
        self.is_done = !self.is_done;
        true
    }
    pub fn task(&self) -> &String {
        &self.task
    }
}

