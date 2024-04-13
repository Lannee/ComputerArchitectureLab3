use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::str::FromStr;

use crate::processor::commands::{DataCommand, Instruction, Mark};

use crate::errors::{LinkError, ParseError};
use crate::machine_code::{Address, Data, Inctructions, MachineCode, RawInctructions};
use crate::input::SourceCode;

use crate::translator::format::*;


pub fn parse(code: &SourceCode) -> Result<(Option<RawInctructions>, Option<Data>), ParseError> {

    let code = code.trim();

    let mut data_labels: HashMap<String, Address> = HashMap::new();
    let mut instructions_labels: HashMap<String, Address> = HashMap::new();

    let byte_counter: RefCell<Address> = 0.into();
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
                    
                    DataCommand::from_str(delate_label(token))
                }).filter(|command| {
                    match command {
                        Err(ParseError::EmptyLineAsCommand) => false,
                        _ => true     
                    }
                }).map(|command| {
                    match command? {
                        DataCommand::Byte(value) => {
                            *byte_counter.borrow_mut() += 1;
                            Ok(vec![value])
                        },
                        _ => Err(ParseError::InstructionInDataSection),
                    }
                }).collect::<Result<Vec<Data>, ParseError>>()
                .map(|vec| {
                    vec.into_iter()
                        .flatten()
                        .collect::<Data>()
                })
            });

    let data = match data {
        Some(res) => Some(res?),
        None => None
    };

    let instruction_counter: RefCell<Address> = 0.into();
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
                    
                    Instruction::from_str(delate_label(token))
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
                }).collect::<Result<Vec<Instruction>, ParseError>>()
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
    
    
    raw_instructions.instructions.iter()
        .map(|instruction| {
            Ok(
                if instruction.can_contain_label() {
                    let mark = instruction.get_mark().unwrap();

                    let label = mark.get_label().unwrap();

                    let address = if instruction.is_data_referencing() {
                            let address = raw_instructions.data_labels.get(label);
                            if let None = address {return Err(LinkError::UndefinedDataLabel(label.to_owned()))}
                            address.unwrap()
                        } else {
                            let address = raw_instructions.instructions_labels.get(label);
                            if let None = address {return Err(LinkError::UndefinedInstructionLabel(label.to_owned()))}
                            address.unwrap()
                        };
                    
                    instruction.set_mark(Mark::Address(*address))?
                } else {
                    instruction.clone()
                }
            )
        }).collect::<Result<Inctructions, LinkError>>()
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
