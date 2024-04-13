use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::str::FromStr;

use crate::processor::commands::Command;

use crate::errors::{LinkError, ParseError};
use crate::machine_code::{Data, Inctructions, MachineCode, RawInctructions};
use crate::input::SourceCode;

use crate::translator::format::*;


pub fn parse(code: &SourceCode) -> Result<(Option<RawInctructions>, Option<Data>), ParseError> {

    let code = code.trim();

    let mut data_labels: HashMap<String, usize> = HashMap::new();
    let mut instructions_labels: HashMap<String, usize> = HashMap::new();

    let byte_counter: RefCell<usize> = 0.into();
    let data = get_section_content(Section::Data, code)
        .map(|data| {
            data.lines()
                .map(|line| get_token(line))
                .filter(|token| token.is_some())
                .map(|token| {
                    let token = token.unwrap();

                    if let Some(data_label) = get_label(token) {
                        data_labels.insert(data_label.to_owned(), *byte_counter.borrow());
                    }
                    
                    Command::from_str(delate_label(token))
                }).filter(|command| {
                    match command {
                        Ok(_) => true,
                        Err(ParseError::EmptyLineAsCommand) => false,
                        Err(_) => true     
                    }
                }).map(|command| {
                    match command? {
                        Command::Byte(value) => {
                            *byte_counter.borrow_mut() += 1;
                            Ok(value)
                        },
                        _ => Err(ParseError::InstructionInDataSection),
                    }
                }).collect::<Result<Data, ParseError>>()
            });

    let data = match data {
        Some(res) => Some(res?),
        None => None
    };

    let instruction_counter: RefCell<usize> = 0.into();
    let instructions = get_section_content(Section::Code, code)
        .map(|code| {
            code.lines()
                .map(|line| get_token(line))
                .filter(|token| token.is_some())
                .map(|token| {
                    let token = token.unwrap();

                    if let Some(instruction_label) = get_label(token) {
                        instructions_labels.insert(instruction_label.to_owned(), *instruction_counter.borrow());
                    }
                    
                    Command::from_str(delate_label(token))
                }).filter(|command| {
                    match command {
                        Ok(_) => true,
                        Err(ParseError::EmptyLineAsCommand) => false,
                        Err(_) => true     
                    }
                }).map(|command| {
                    let command = command?;
                    *instruction_counter.borrow_mut() += 1;
                    Ok(command)
                }).collect::<Result<Vec<Command>, ParseError>>()
        });

    let instructions = match instructions {
        Some(res) => Some(res?),
        None => None
    };

    Ok((
        instructions.map(|instructions| {
            RawInctructions::new(instructions, instructions_labels, data_labels)
        }), 
        data
    ))
}


pub fn link(raw_instructions: RawInctructions) -> Result<Inctructions, LinkError> {

    println!("instructions_labels: {:?}", raw_instructions.instructions_labels);
    println!("data_labels: {:?}", raw_instructions.data_labels);

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

fn delate_label(token: &str) -> &str {
    match token.split_once(LABEL) {
        Some((_, a)) => a.trim(),
        None => token
    }
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
