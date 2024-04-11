use crate::processor::commands::Command;

use serde::Serialize;


pub type Inctructions = Vec<Command>;
pub type Data = Vec<u8>;
pub type Labels = Vec<String>;


#[derive(Serialize)]
pub struct MachineCode {
    instructions: Option<Inctructions>,
    data: Option<Data>
}


impl MachineCode {
    pub fn new(instructions: Option<Inctructions>, data: Option<Data>) -> MachineCode {
        MachineCode {
            instructions,
            data
        }
    }
}


pub struct RawInctructions {
    pub instructions: Inctructions,
    pub labels: Labels
}

impl RawInctructions {
    pub fn new(instructions: Inctructions, labels: Labels) -> RawInctructions {
        RawInctructions {
            instructions,
            labels
        }
    }
}