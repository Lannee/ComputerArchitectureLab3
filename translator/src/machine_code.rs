use crate::processor::commands::Instruction;
use std::collections::HashMap;

use serde::Serialize;


pub type Inctructions = Vec<Instruction>;
pub type Data = Vec<u8>;
pub type Labels = HashMap<Label, Address>;

pub type Address = u32;
pub type Label = String;


#[derive(Serialize)]
pub struct MachineCode {
    instructions: Option<Inctructions>,
    data: Option<Data>,
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
    pub instructions_labels: Labels,
    pub data_labels: Labels,
}

impl RawInctructions {
    pub fn new(instructions: Inctructions, instructions_labels: Labels, data_labels: Labels) -> RawInctructions {
        RawInctructions {
            instructions,
            instructions_labels,
            data_labels
        }
    }
}