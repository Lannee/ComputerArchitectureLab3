use crate::{errors::TranslationError, input::SourceCode, machine_code::MachineCode};

pub mod parser;
pub mod format;


pub fn translate(code: &SourceCode) -> Result<MachineCode, TranslationError> {
    parser::link(
        parser::parse(code)
            .map_err(|err| TranslationError::ParseError(err))?
        ).map_err(|err| TranslationError::LinkError(err))
}