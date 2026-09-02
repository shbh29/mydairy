use std::io;
use std::io::Write;
use crate::cmds::cmds;

mod errors;
mod cmds;

fn main() {
    let mut input = String::new();

    loop {
        print!("dairy> ");
        let _ = io::stdout().flush();
        if io::stdin().read_line(&mut input)
            .is_err() {
                eprintln!("Error taking input");
                continue;
        }
        if input.trim().is_empty() {
            break;
        }

        if cmds(input.trim()).is_err() {
            eprintln!("Error executing command. Please try again");
            continue;
        }

        input.clear();
    }
        
    println!("Hello, world!");
}
