use core::fmt;
use std::{num::IntErrorKind, str::FromStr};

use crate::errors::LinkError;
use crate::{errors::ParseError, machine_code::Label};
use crate::machine_code::{Address, PortIndex};
use crate::translator::format::*;
use crate::processor::PROCESSOR;

use super::GlobRegister;

use regex::Regex;
use serde::{Serialize, Serializer};



#[derive(Serialize, Clone)]
pub enum Instruction {
    Mov(&'static GlobRegister, &'static GlobRegister),
    Movn(&'static GlobRegister, i32),

    // In(&'static GlobRegister, &'static GlobRegister),
    Out(PortIndex, &'static GlobRegister),

    Jmp(Mark<Address>),
    Be(Mark<Address>),
    Bg(Mark<Address>),

    La(&'static GlobRegister, Mark<Address>),
    Lw(&'static GlobRegister, Mark<Address>),
    Lb(&'static GlobRegister, Mark<Address>),
    Lbu(&'static GlobRegister, Mark<Address>),
    Stw(Mark<Address>, &'static GlobRegister),
    Stb(Mark<Address>, &'static GlobRegister),

    Add(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Sub(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Mul(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Rem(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Cmp(&'static GlobRegister, &'static GlobRegister),

    Nop,
    Halt
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
            "be" => Self::init_be(args),
            "bg" => Self::init_bg(args),

            "la" => Self::init_la(args),
            "lw" => Self::init_lw(args),
            "lb" => Self::init_lb(args),
            "lbu" => Self::init_lbu(args),
            "stw" => Self::init_stw(args),
            "stb" => Self::init_stb(args),

            "add" => Self::init_add(args),
            "sub" => Self::init_add(args),
            "mul" => Self::init_add(args),
            "rem" => Self::init_add(args),
            "cmp" => Self::init_cmp(args),

            "nop" => Ok(Self::Nop),
            "halt" => Ok(Self::Halt),

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
            Lw(_, _) => true,
            Lb(_, _) => true,
            Lbu(_, _) => true,
            Stw(_, _) => true,
            Stb(_, _) => true,

            _ => false
        }
    }

    pub fn is_instruction_referencing(&self) -> bool {
        use Instruction::*;
        match self {
            Jmp(_) => true,
            Be(_) => true,
            Bg(_) => true,
            _ => false
        }
    }

    pub fn get_mark(&self) -> Option<&Mark<Address>> {
        use Instruction::*;
        match self {
            Jmp(mark) => Some(mark),
            Be(mark) => Some(mark),
            Bg(mark) => Some(mark),
            La(_, mark) => Some(mark),
            Lw(_, mark) => Some(mark),
            Lb(_, mark) => Some(mark),
            Lbu(_, mark) => Some(mark),
            Stw(mark, _) => Some(mark),
            Stb(mark, _) => Some(mark),
            _ => None
        }
    }

    pub fn set_mark(&self, mark: Mark<Address>) -> Result<Instruction, LinkError> {
        use Instruction::*;
        match self {
            Jmp(_) => Ok(Jmp(mark)),
            Be(_) => Ok(Be(mark)),
            Bg(_) => Ok(Bg(mark)),
            La(target, _) => Ok(La(target, mark)),
            Lw(target, _) => Ok(Lw(target, mark)),
            Lb(target, _) => Ok(Lb(target, mark)),
            Lbu(target, _) => Ok(Lbu(target, mark)),
            Stw(_, source) => Ok(Stw(mark, source)),
            Stb(_, source) => Ok(Stb(mark, source)),
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

        Ok(Instruction::Out(
                args[0].parse()
                    .map_err(|_| ParseError::InvalidCommandArgumants)?, 
            get_register(args[1])?))
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

    fn init_be(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Be(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Be(Mark::Label(args[0].to_owned())))
            }
        }
    }

    fn init_bg(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Bg(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Bg(Mark::Label(args[0].to_owned())))
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

    fn init_lw(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let target = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Lw(target, Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Lw(target, Mark::Label(args[1].to_owned())))
            }
        }
    }

    fn init_lb(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let target = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Lb(target, Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Lb(target, Mark::Label(args[1].to_owned())))
            }
        }
    }

    fn init_lbu(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let target = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Lbu(target, Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Lbu(target, Mark::Label(args[1].to_owned())))
            }
        }
    }

    fn init_stw(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let source = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Stw(Mark::Address(ok), source)),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Stw(Mark::Label(args[1].to_owned()), source))
            }
        }
    }

    fn init_stb(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        let source = get_register(args[0])?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Stb(Mark::Address(ok), source)),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Stb(Mark::Label(args[1].to_owned()), source))
            }
        }
    }

    fn init_add(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 3)?;

        Ok(Instruction::Add(
            get_register(args[0])?, 
            get_register(args[1])?, 
            get_register(args[2])?
        ))
    }

    fn init_sub(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 3)?;

        Ok(Instruction::Sub(
            get_register(args[0])?, 
            get_register(args[1])?, 
            get_register(args[2])?
        ))
    }

    fn init_mul(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 3)?;

        Ok(Instruction::Mul(
            get_register(args[0])?, 
            get_register(args[1])?, 
            get_register(args[2])?
        ))
    }

    fn init_rem(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 3)?;

        Ok(Instruction::Rem(
            get_register(args[0])?, 
            get_register(args[1])?, 
            get_register(args[2])?
        ))
    }

    fn init_cmp(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Cmp(
            get_register(args[0])?, 
            get_register(args[1])?, 
        ))
    }

}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Mov(target, from) => write!(f, ""),
            Movn(target, value) => write!(f, ""),
            // In(target, from) => write!(f, ""),
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