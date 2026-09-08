use crate::cmds::Cmds;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use mydlib::errors::Errors;

/*Using this trait should save the content to the current file.*/
pub trait WriteContent {
    fn write(file_content: &str) -> Result<bool, Errors>;
}

impl WriteContent for Cmds {

    fn write(file_content: &str) -> Result<bool, Errors> {
        let file = File::create("current.json")
                .map_err(Errors::InvalidFileCreation)?;
        let mut bufw = BufWriter::new(file);

        bufw.write_all(file_content.as_bytes())
            .map_err(Errors::InvalidFileWriteOperation)
            .map(|_| true)
    }
}


