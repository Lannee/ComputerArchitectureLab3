use crate::{errors::TranslationError, input::SourceCode, machine_code::MachineCode};

pub mod parser;
mod format;


pub fn translate(code: SourceCode) -> Result<MachineCode, TranslationError> {
    todo!()
}