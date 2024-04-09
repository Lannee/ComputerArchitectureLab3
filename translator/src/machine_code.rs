use crate::processor::commands::Command;

use serde::Serialize;


pub type Inctructions = Vec<Command>;
pub type Data = Vec<u8>;


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
    instructions: Inctructions,
    labels: Vec<String>
}