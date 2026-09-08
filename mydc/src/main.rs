use std::io;
use std::io::Write;
use crate::cmds::cmds;

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

        if let Err(e) = cmds(input.trim())  {
            e.print_stack_trace();
        }

        input.clear();
    }
        
    println!("Hello, world!");
}
