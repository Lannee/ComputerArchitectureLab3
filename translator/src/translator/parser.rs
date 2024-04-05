use crate::machine_code::MachineCode;
use crate::{errors::TranslationError, input::SourceCode};

use crate::translator::format::*;

pub fn parse(code: SourceCode) -> Result<MachineCode, TranslationError> {

    todo!()
}


fn get_token(line: String) -> String {
    line
        .split(COMMENT)
        .next()
        .unwrap()
        .trim()
        .to_string()
}

fn get_token_elements<'a>(token: &'a String) -> (&'a str, Vec<&'a str>) {

    let instr_args = token.split_once(INSTRUCTION_ARGUMENTS_SEPARATOR);

    match instr_args {
        Some(instr_args) => (
            instr_args.0, 
            instr_args.1
                .split(ARGUMENTS_SEPARATOR)
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
        ),
        None => (token.as_str(), Vec::new())
    }
}


pub fn get_section_content(section: &Section, code: &String) -> Option<String> {
    Some(code
            .split(&format!("{}", SECTION))
            .map(|x| x.trim())
            .filter(|&x| x.starts_with(&section.to_string()))
            .next()?
            .strip_prefix(&section.to_string())
            .unwrap()
            .trim()
            .to_string()
        )
}