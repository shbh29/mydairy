use crate::errors::Errors;

pub fn cmds(input: &str) -> Result<bool, Errors> {
    println!("input: {:?}", input);
        let (cmd, _arg1, _arg2): (Cmds, String, String) = filter(&input)?;
    match cmd {
        Cmds::FreeWrite(text) => {
            println!("The file text is: {:?}", text);
            Ok(true)
        }
    }
}

enum Cmds {
    FreeWrite(String)
}

fn filter(input: &str) -> Result<(Cmds, String, String), Errors> {
    let inpts: Vec<_> = input.split(" ").collect();
    println!("inpts: {:?}", inpts);

    let cmd : Option<_> = inpts.get(0);

    match cmd {
        Some(_cmd) => {
            Ok((Cmds::FreeWrite(String::from("fw text")), String::new(), String::new()))
        },
        None => {
            Err(Errors::InvalidInput)
        }
    }
}            

