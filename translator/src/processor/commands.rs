use core::fmt;
use std::{num::IntErrorKind, str::FromStr};

use crate::errors::LinkError;
use crate::{errors::ParseError, machine_code::Label};
use crate::machine_code::Address;
use crate::translator::format::*;
use crate::processor::PROCESSOR;

use super::GlobRegister;

use regex::Regex;
use serde::{Serialize, Serializer};

#[derive(Serialize, Clone)]
pub enum Instruction {
    Mov(&'static GlobRegister, &'static GlobRegister),
    Movn(&'static GlobRegister, i32),

    In(&'static GlobRegister, &'static GlobRegister),
    Out(&'static GlobRegister, &'static GlobRegister),

    Jmp(Mark<Address>),

    La(&'static GlobRegister, Mark<Address>)
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        println!("command: \"{raw_command}\"\nargs: {:?}", token_elements.1);

        let args = token_elements.1.as_slice();
        match token_elements.0 {
            "mov" => Self::init_mov(args),
            "movn" => Self::init_movn(args),
            "out" => Self::init_out(args),
            "jmp" => Self::init_jmp(args),

            "la" => Self::init_la(args),

            "" => Err(ParseError::EmptyLineAsCommand),
            _ => Err(ParseError::NoSuchCommand(token_elements.0.to_owned()))
        }
    }
}

impl Instruction {
    pub fn can_contain_label(&self) -> bool {
        self.is_data_referencing() || self.is_instruction_referencing()
    }

    pub fn is_data_referencing(&self) -> bool {
        use Instruction::*;
        match self {
            La(_, _) => true,
            _ => false
        }
    }

    pub fn is_instruction_referencing(&self) -> bool {
        use Instruction::*;
        match self {
            Jmp(_) => true,
            _ => false
        }
    }

    pub fn get_mark(&self) -> Option<&Mark<Address>> {
        use Instruction::*;
        match self {
            Jmp(mark) => Some(mark),
            La(_, mark) => Some(mark),
            _ => None
        }
    }

    pub fn set_mark(&self, mark: Mark<Address>) -> Result<Instruction, LinkError> {
        use Instruction::*;
        match self {
            Jmp(_) => Ok(Jmp(mark)),
            La(target, _) => Ok(La(target, mark)),
            _ => Err(LinkError::UnmarkableInstruction)
        }
    }
}

impl Instruction {
    fn init_mov(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Mov(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_movn(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Movn(
            get_register(args[0])?, 
            args[1].parse()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
        ))
    }

    fn init_out(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Out(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_jmp(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Jmp(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Jmp(Mark::Label(args[0].to_owned())))
            }
        }

    }

    fn init_la(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let target = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::La(target, Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::La(target, Mark::Label(args[1].to_owned())))
            }
        }

    }

}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Mov(target, from) => write!(f, ""),
            Movn(target, value) => write!(f, ""),
            In(target, from) => write!(f, ""),
            Out(target, from) => write!(f, ""),
            _ => write!(f, ""),
        }  
    }    
}

fn get_token_elements(token: &str) -> (&str, Vec<&str>) {
    let instr_args = token.split_once(INSTRUCTION_ARGUMENTS_SEPARATOR);
    // let separator: Regex = Regex::new(r#"("[^"]+"|[^,"]+)"#).unwrap();

    match instr_args {
        Some(instr_args) => (
            instr_args.0, 
            instr_args.1
                .split(ARGUMENTS_SEPARATOR)
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
            // {println!("{:?}", separator.captures(instr_args.1).unwrap().iter().map(|mat| mat.unwrap().range()).collect::<Vec<_>>()); vec![]}
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
        Err(ParseError::InvalidAmountOfCommandArguments(args.into_iter().map(|str| str.to_string()).collect(), expexted, args.len()))
    } else {Ok(())}
}



pub enum DataCommand {
    Byte(u8),
    Str(String)
}

impl FromStr for DataCommand {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        println!("command: \"{raw_command}\"\nargs: {:?}", token_elements.1);

        let args = token_elements.1.as_slice();
        match token_elements.0 {
            "byte" => Self::init_byte(args),
            "char" => Self::init_char(args),
            "str" => Self::init_str(args),

            "" => Err(ParseError::EmptyLineAsCommand),
            _ => Err(ParseError::NoSuchCommand(token_elements.0.to_owned()))
        }
    }
}


impl DataCommand {
    fn init_byte(args: &[&str]) -> Result<DataCommand, ParseError> {
        check_args_len(args, 1)?;

        Ok(DataCommand::Byte(
            args[0].parse()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
        ))
    }

    fn init_char(args: &[&str]) -> Result<DataCommand, ParseError> {
        check_args_len(args, 1)?;

        Ok(DataCommand::Byte(
            args[0]
                .parse::<char>()
                .map_err(|_| ParseError::InvalidCommandArgumants)?
                as u8
        ))
    }

    fn init_str(args: &[&str]) -> Result<DataCommand, ParseError> {
        check_args_len(args, 1)?;

        Ok(DataCommand::Str(
            args[0].to_owned()
        ))
    }
}



#[derive(Clone)]
pub enum Mark<T> {
    Label(String),
    Address(T)
}

impl<T> Mark<T> {
    pub fn is_label(&self) -> bool {
        match self {
            Mark::Label(_) => true,
            _ => false
        }
    }

    pub fn get_label(&self) -> Option<&str> {
        match self {
            Mark::Label(label) => Some(label),
            Mark::Address(_) => None
        }
    }
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