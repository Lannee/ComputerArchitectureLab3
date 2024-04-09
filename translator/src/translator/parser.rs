use crate::errors::{LinkError, ParseError};
use crate::machine_code::{Data, Inctructions, RawInctructions};
use crate::input::SourceCode;

use crate::translator::format::*;


pub fn parse(code: &SourceCode) -> Result<(Option<RawInctructions>, Option<Data>), ParseError> {

    let code = code.trim();

    let data = get_section_content(Section::Data, code)
        .map(|data| {
            "hello".as_bytes().to_vec()
        });

    let instructions = get_section_content(Section::Code, code)
        .map(|code| {

        });


    todo!()
}


pub fn link(raw_instructions: RawInctructions) -> Result<Inctructions, LinkError> {
    todo!()
}


fn get_token(line: &str) -> &str {
    line
        .split(COMMENT)
        .next()
        .unwrap()
        .trim()
}

fn get_token_elements(token: &str) -> (&str, Vec<&str>) {

    let instr_args = token.split_once(INSTRUCTION_ARGUMENTS_SEPARATOR);

    match instr_args {
        Some(instr_args) => (
            instr_args.0, 
            instr_args.1
                .split(ARGUMENTS_SEPARATOR)
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
        ),
        None => (token, Vec::new())
    }
}

fn get_label(token: &str) -> Option<&str> {
    token.split_once(LABEL)
        .map(|split| {
            split.0
                .trim()
        })
}

pub fn get_section_content(section: Section, code: &str) -> Option<String> {
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
