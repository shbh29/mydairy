use serde::{Deserialize, Serialize};


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
    pub fn push_file_content(&mut self, file_content: &String) {
        self.content.push_str(&file_content);
    }
    pub fn content(&self) -> &String {
        &self.content
    }
}

