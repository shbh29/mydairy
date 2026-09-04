use crate::errors::Errors;
use crate::cmds::free_write::FreeWrite;
use crate::cmds::intentions::HandleIntentions;
use crate::cmds::intentions::Intent;
use crate::models::CheckList;

mod free_write;
mod write_content;
mod intentions;

/* Objective of this function is to delgate command to handlers */
pub fn cmds(input: &str) -> Result<bool, Errors> {
    println!("input: {:?}", input);
    
    match filter(&input) {
        Ok(cmd) => match cmd {
            Cmds::FreeWrite => cmd.handle_free_write(),
            Cmds::Intentions(ref intent) => cmd.handle_intentions(intent.clone()),
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        },  
    }
}

pub enum Cmds {
    FreeWrite,
    Intentions(Intent)
}

/* Objective of this method is to select the Cmd from the first string. */
fn filter(input: &str) -> Result<Cmds, Errors> {
    let inpts: Vec<_> = input.split(" ").collect();
    println!("inpts: {:?}", inpts);

    let cmd : Option<_> = inpts.get(0);

    match cmd {
        Some(cmd) if *cmd == "fw" => {
            Ok(Cmds::FreeWrite)
        },
        Some(cmd) if *cmd == "in" => {
            Ok(Cmds::Intentions(Intent::new()))
        },
        None => {
            Err(Errors::InvalidInput)
        },
        _ => Err(Errors::InvalidCmd)
    }
}            

