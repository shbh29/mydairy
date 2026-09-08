use mydlib::errors::Errors;
use crate::cmds::free_write::HandleFreeWrite;
use crate::cmds::intentions::HandleIntentions;

pub mod free_write;
mod write_content;
pub mod intentions;

/* Objective of this function is to delgate command to handlers */
pub fn cmds(input: &str) -> Result<bool, Errors> {
    println!("input: {:?}", input);
    
    match filter(&input) {
        Ok(cmd) => match cmd {
            Cmds::FreeWrite => cmd.handle_free_write(),
            Cmds::Intentions(inpts) => Cmds::handle_intentions(inpts),
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        },  
    }
}

pub enum Cmds {
    FreeWrite,
    Intentions(Vec<String>)
}

/* Objective of this method is to select the Cmd from the first string. */
fn filter(input: &str) -> Result<Cmds, Errors> {
    let inpts: Vec<String> = input.split(" ")
            .map(|s| String::from(s))
            .collect();
    println!("inpts: {:?}", inpts);
    
    let cmd : Option<_> = inpts.get(0);

    match cmd {
        Some(cmd) if *cmd == "fw" => {
            Ok(Cmds::FreeWrite)
        },
        Some(cmd) if *cmd == "in" => {
            Ok(Cmds::Intentions(inpts))
        },
        None => {
            Err(Errors::InvalidInput)
        },
        _ => Err(Errors::InvalidCmd)
    }
}            

