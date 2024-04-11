use crate::{errors::TranslationError, input::SourceCode, machine_code::MachineCode};

pub mod parser;
pub mod format;


pub fn translate(code: &SourceCode) -> Result<MachineCode, TranslationError> {
    let parse_result = parser::parse(code)
        .map_err(|err| TranslationError::ParseError(err))?;

    Ok(MachineCode::new(
        match parse_result.0.map(|x| parser::link(x)) {
            Some(res) => Some(res.map_err(|err| TranslationError::LinkError(err))?),
            None => None
        },
        parse_result.1
    ))
}