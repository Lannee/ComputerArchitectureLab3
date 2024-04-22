use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::str::FromStr;

use crate::processor::commands::{DataCommand, Instruction, Mark};

use crate::errors::{LinkError, ParseError};
use crate::machine_code::{Address, Data, Inctructions, MachineCode, RawInctructions};
use crate::input::SourceCode;

use crate::translator::format::*;


pub fn parse(code: &SourceCode) -> Result<RawInctructions, ParseError> {

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
                    let command = command?;
                    *byte_counter.borrow_mut() += command.get_size() as u32;
                    Ok(command)
                }).collect::<Result<Vec<DataCommand>, ParseError>>()
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

    // Ok((
    //     instructions.map(|instructions| {
    //         RawInctructions::new(instructions, instructions_labels, data_labels)
    //     }), 
    //     data
    // ))
    Ok(RawInctructions::new(instructions, data, instructions_labels, data_labels))
}


pub fn link(raw_instructions: RawInctructions) -> Result<MachineCode, LinkError> {
    let instructions = 
        raw_instructions.instructions
            .map(|instructions| {
                instructions.iter()
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
            });
    let instructions = match instructions {
        Some(instructions) => Some(instructions?),
        None => None
    };

    let data = 
        raw_instructions.data
            .map(|commands| {
                commands.iter()
                    .map(|command| {
                        Ok(
                            if command.can_contain_label() {
                                let mark = command.get_mark().unwrap();
            
                                let label = mark.get_label().unwrap();
            
                                let address = if command.is_data_referencing() {
                                        let address = raw_instructions.data_labels.get(label);
                                        if let None = address {return Err(LinkError::UndefinedDataLabel(label.to_owned()))}
                                        address.unwrap()
                                    } else {
                                        let address = raw_instructions.instructions_labels.get(label);
                                        if let None = address {return Err(LinkError::UndefinedInstructionLabel(label.to_owned()))}
                                        address.unwrap()
                                    };
                                
                                command.set_mark(Mark::Address(*address))?
                            } else {
                                command.clone()
                            }
                        )
                    })
                    .map(|command| {
                        match command? {
                            DataCommand::Byte(value) => {
                                Ok(vec![value])
                            },
                            DataCommand::Str(str) => {
                                let c_fmt_str = CString::new(str).unwrap();
                                let bytes = c_fmt_str.as_bytes_with_nul();
                                Ok(bytes.to_vec())
                            },
                            DataCommand::Vec(address) => {
                                match address {
                                    Mark::Address(addr) => Ok(addr.to_le_bytes().to_vec()),
                                    Mark::Label(label) => Err(LinkError::UnresolvedInstruction(label))
                                }
                            }
                        }
                    }).collect::<Result<Vec<Data>, LinkError>>()
                    .map(|vec| {
                        vec.into_iter()
                            .flatten()
                            .collect::<Data>()
                    })
            });
    let data = match data {
        Some(data) => Some(data?),
        None => None
    };

    Ok(MachineCode::new(instructions, data))

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
