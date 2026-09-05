
#[derive(Debug)]
pub enum Errors {
    InvalidInput,
    IOError,
    InvalidCmd,
    InvalidFileCreation(std::io::Error),
    InvalidFileWriteOperation(std::io::Error),
    InvalidSerializeOp,
}

impl Errors {
    pub fn print_stack_trace(&self) {
        match self {
            Errors::InvalidFileCreation(err) |
            Errors::InvalidFileWriteOperation(err) => {
                eprintln!("Error performing operation: {:?}", err);
            },
            Errors::InvalidCmd |
            Errors::InvalidInput |
            Errors::IOError |
            Errors::InvalidSerializeOp => {
                eprintln!("Error performing operation: {:?}", self);
            }
        };
    }
}
