use crate::processor::commands::Command;

use serde::Serialize;

#[derive(Serialize)]
pub struct MachineCode {
    instructions: Vec<Command>,
    data: Vec<u8>
}