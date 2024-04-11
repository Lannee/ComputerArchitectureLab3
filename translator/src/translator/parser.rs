use std::str::FromStr;

use crate::processor::commands::Command;

use crate::errors::{LinkError, ParseError};
use crate::machine_code::{Data, Inctructions, MachineCode, RawInctructions};
use crate::input::SourceCode;

use crate::translator::format::*;


pub fn parse(code: &SourceCode) -> Result<(Option<RawInctructions>, Option<Data>), ParseError> {

    let code = code.trim();

    let mut data_labels: Vec<&str> = Vec::new();
    let mut instructions_labels: Vec<String> = Vec::new();

    let data = get_section_content(Section::Data, code)
        .map(|data| {
            "hello".as_bytes().to_vec()
        });

    let instructions = get_section_content(Section::Code, code)
        .map(|code| {
            code.lines()
                .map(|line| get_token(line))
                .filter(|token| token.is_some())
                .map(|token| {
                    let token = token.unwrap();
                    
                    if let Some(instruction_label) = get_label(token) {
                        todo!("Do instruction label save")
                    }
                    
                    Command::from_str(token)
                }).collect::<Result<Vec<Command>, ParseError>>()
        });

    let instructions = match instructions {
        Some(res) => Some(res?),
        None => None
    };


    
    Ok((
        instructions.map(|instructions| {
            RawInctructions::new(instructions, instructions_labels)
        }), 
        data
    ))
}


pub fn link(raw_instructions: RawInctructions) -> Result<Inctructions, LinkError> {
    Ok(
        raw_instructions.instructions
    )
}


fn get_token(line: &str) -> Option<&str> {
    let token = line
        .split(COMMENT)
        .next()
        .unwrap()
        .trim();

    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn get_label(token: &str) -> Option<&str> {
    token.split_once(LABEL)
        .map(|split| {
            split.0
                .trim()
        })
}

pub fn get_section_content(section: Section, code: &str) -> Option<&str> {
    let content = code.split(&format!("{}", SECTION))
        .map(|x| x.trim())
        .filter(|&x| x.starts_with(&section.to_string()))
        .next()?
        .strip_prefix(&section.to_string())
        .unwrap()
        .trim();

    if content.is_empty() {
        None
    } else {
        Some(content)
    }
}
