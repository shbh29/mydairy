
#[derive(Clone)]
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
}
