use core::fmt;
use std::str::FromStr;

use crate::errors::ParseError;
use crate::translator::format::*;
use crate::processor::PROCESSOR;

use super::GlobRegister;

use serde::Serialize;

#[derive(Serialize)]
pub enum Command {
    MovRegReg(&'static GlobRegister, &'static GlobRegister),
    MovRegVal(&'static GlobRegister, i32),

    In(&'static GlobRegister, &'static GlobRegister),
    Out(&'static GlobRegister, &'static GlobRegister),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        match token_elements.0 {
            "mov" => Self::init_mov(token_elements.1.as_slice()),
            "movn" => Self::init_movn(token_elements.1.as_slice()),
            _ => Err(ParseError::NoSuchCommand(token_elements.0.to_owned()))
        }
    }
}

impl Command {
    fn init_mov(args: &[&str]) -> Result<Command, ParseError> {
        if args.len() == 2 {
            Ok(Command::MovRegReg(get_register(args[0])?, get_register(args[1])?))
        } else {
            Err(ParseError::InvalidCommandArgumants)
        }
    }

    fn init_movn(args: &[&str]) -> Result<Command, ParseError> {
        if args.len() == 2 {
            Ok(Command::MovRegVal(
                get_register(args[0])?, 
                args[1].parse()
                    .map_err(|_| ParseError::InvalidCommandArgumants)?
            ))
        } else {
            Err(ParseError::InvalidCommandArgumants)
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;

        match self {
            MovRegReg(target, from) => write!(f, ""),
            MovRegVal(target, value) => write!(f, ""),
            In(target, from) => write!(f, ""),
            Out(target, from) => write!(f, "")
        }  
    }    
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

fn get_register(reg_name: &str) -> Result<&'static GlobRegister, ParseError> {
    match PROCESSOR.get_register(reg_name) {
        Some(reg) => Ok(reg),
        None => return Err(ParseError::InvalidCommandArgumants)
    }
}