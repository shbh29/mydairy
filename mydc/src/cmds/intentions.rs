use mydlib::errors::Errors;
use crate::cmds::Cmds;
use mydlib::models::journal_entry::JournalEntry;
use crate::cmds::write_content::WriteContent;
use mydlib::models::intentions::Intentions;

pub trait HandleIntentions {
    fn handle_intentions(inpts: Vec<String>) -> Result<bool, Errors>;
}

impl HandleIntentions for Cmds {
    fn handle_intentions(inpts: Vec<String>) -> Result<bool, Errors> {
        let mut je = JournalEntry::load()?;
        println!("Intentions: ");

        let intent: &mut Intentions = je.intentions_mut();

        let sub_cmd = inpts.get(1)
            .map(|s| s.as_str());

        let _ = match sub_cmd {
            Some("add") => intent.add(inpts[2..].join(" ")),
            Some("rm") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                // this can throw invalid index exception
                intent.remove(index - 1)
            },
            Some("dn") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                intent.toggle_done(index - 1)
            },
            Some("mvcf") => {
                let index: usize = inpts[2].parse()
                        .map_err(|_| Errors::InvalidCmd)?;
                intent.move_to_cf(index - 1)
            },
            None | _ => {
                return Err(Errors::InvalidCmd);
            }
        };

        intent.display();
        
        let je_str = serde_json::to_string(&je)
            .map_err(|_| Errors::InvalidSerializeOp)?;

        Cmds::write(&je_str)
    }
}


