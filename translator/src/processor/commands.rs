use core::fmt;
use std::{io::Empty, num::IntErrorKind, str::FromStr};

use crate::errors::ParseError;
use crate::machine_code::Address;
use crate::translator::format::*;
use crate::processor::PROCESSOR;

use super::GlobRegister;

use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub enum Command {
    Mov(&'static GlobRegister, &'static GlobRegister),
    Movn(&'static GlobRegister, i32),

    In(&'static GlobRegister, &'static GlobRegister),
    Out(&'static GlobRegister, &'static GlobRegister),

    Byte(u8),

    Jmp(Mark<Address>)
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        let args = token_elements.1.as_slice();
        match token_elements.0 {
            "mov" => Self::init_mov(args),
            "movn" => Self::init_movn(args),
            "out" => Self::init_out(args),
            "byte" => Self::init_byte(args),
            "char" => Self::init_char(args),
            "jmp" => Self::init_jmp(args),

            "" => Err(ParseError::EmptyLineAsCommand),
            _ => Err(ParseError::NoSuchCommand(token_elements.0.to_owned()))
        }
    }
}

impl Command {
    pub fn is_data_command(&self) -> bool {
        use Command::*;

        match self {
            Byte(_) => true,
            _ => false
        }
    }
}

impl Command {
    fn init_mov(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 2)?;

        Ok(Command::Mov(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_movn(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 2)?;

        Ok(Command::Movn(
            get_register(args[0])?, 
            args[1].parse()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
        ))
    }

    fn init_out(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 2)?;

        Ok(Command::Out(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_byte(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 1)?;

        Ok(Command::Byte(
            args[0].parse()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
        ))
    }

    fn init_char(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 1)?;

        Ok(Command::Byte(
            args[0]
                .parse::<char>()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
                as u8
        ))
    }

    fn init_jmp(args: &[&str]) -> Result<Command, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Command::Jmp(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Command::Jmp(Mark::Label(args[0].to_owned())))
            }
        }

    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;

        match self {
            Mov(target, from) => write!(f, ""),
            Movn(target, value) => write!(f, ""),
            In(target, from) => write!(f, ""),
            Out(target, from) => write!(f, ""),

            Byte(value) => write!(f, ""),
            _ => write!(f, ""),
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

fn check_args_len(args: &[&str], expexted: usize) -> Result<(), ParseError> {
    if args.len() != expexted {
        Err(ParseError::InvalidAmountOfCommandArguments(args.into_iter().map(|str| str.to_string()).collect(), 2, args.len()))
    } else {Ok(())}
}



pub enum Mark<T> {
    Label(String),
    Address(T)
}

impl<T: Serialize> Serialize for Mark<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where S: Serializer { 
        match self {
            Mark::Address(address) => address.serialize(serializer),
            Mark::Label(_) => panic!("Label serialization")
        }
    }
}