
#[derive(Debug)]
pub enum Errors {
    InvalidInput,
    IOError,
    InvalidCmd,
    InvalidFileCreation(std::io::Error),
    InvalidFileWriteOperation(std::io::Error),
    InvalidSerializeOp,
    InvalidDeserializeOp(std::io::Error),
    InvalidJsonInFile(serde_json::Error),
    UnableToOpenFile,
}

impl Errors {
    pub fn print_stack_trace(&self) {
        match self {
            Errors::InvalidFileCreation(err) |
            Errors::InvalidDeserializeOp(err) |
            Errors::InvalidFileWriteOperation(err) => {
                eprintln!("Error performing operation: {:?}", err);
            },
            Errors::InvalidJsonInFile(err) => {
                eprintln!("Error performing operation: {:?}", err);
            },
            Errors::InvalidCmd |
            Errors::InvalidInput |
            Errors::IOError |
            Errors::UnableToOpenFile |
            Errors::InvalidSerializeOp => {
                eprintln!("Error performing operation: {:?}", self);
            }
        };
    }
}
