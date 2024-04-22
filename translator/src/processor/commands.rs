use std::{num::IntErrorKind, str::FromStr};

use crate::errors::LinkError;
use crate::errors::ParseError;
use crate::machine_code::{Address, PortIndex};
use crate::translator::format::*;
use crate::processor::PROCESSOR;

use super::GlobRegister;

use serde::{Serialize, Serializer};



#[derive(Serialize, Clone)]
pub enum Instruction {
    Mov(&'static GlobRegister, &'static GlobRegister),
    Movn(&'static GlobRegister, i32),

    In(&'static GlobRegister, PortIndex),
    Out(PortIndex, &'static GlobRegister),
    Di,
    Ei,

    Jmp(Mark<Address>),
    Be(Mark<Address>),
    Bne(Mark<Address>),
    Bg(Mark<Address>),
    Ble(Mark<Address>),

    La(&'static GlobRegister, Mark<Address>),
    Lw(&'static GlobRegister, Mark<Address>),
    Lb(&'static GlobRegister, Mark<Address>),
    Lbi(&'static GlobRegister, &'static GlobRegister),
    Lbu(&'static GlobRegister, Mark<Address>),
    Stw(Mark<Address>, &'static GlobRegister),
    Stb(Mark<Address>, &'static GlobRegister),
    Stwi(&'static GlobRegister, &'static GlobRegister),
    Stbi(&'static GlobRegister, &'static GlobRegister),

    Inc(&'static GlobRegister),
    Add(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Sub(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Mul(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Rem(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    And(&'static GlobRegister, &'static GlobRegister, &'static GlobRegister),
    Cmp(&'static GlobRegister, &'static GlobRegister),
    Test(&'static GlobRegister, &'static GlobRegister),

    Call(Mark<Address>),
    Ret,
    Push(&'static GlobRegister),
    Pop(&'static GlobRegister),

    Nop,
    Halt
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        let args = token_elements.1.as_slice();
        match token_elements.0 {
            "mov" => Self::init_mov(args),
            "movn" => Self::init_movn(args),

            "in" => Self::init_in(args),
            "out" => Self::init_out(args),
            "di" => Ok(Self::Di),
            "ei" => Ok(Self::Ei),

            "jmp" => Self::init_jmp(args),
            "be" => Self::init_be(args),
            "bne" => Self::init_bne(args),
            "bg" => Self::init_bg(args),
            "ble" => Self::init_ble(args),

            "la" => Self::init_la(args),
            "lw" => Self::init_lw(args),
            "lb" => Self::init_lb(args),
            "lbi" => Self::init_lbi(args),
            "lbu" => Self::init_lbu(args),
            "stw" => Self::init_stw(args),
            "stb" => Self::init_stb(args),
            "stwi" => Self::init_stwi(args),
            "stbi" => Self::init_stbi(args),

            "inc" => Self::init_inc(args),
            "add" => Self::init_add(args),
            "sub" => Self::init_sub(args),
            "mul" => Self::init_mul(args),
            "rem" => Self::init_rem(args),
            "and" => Self::init_and(args),
            "cmp" => Self::init_cmp(args),
            "test" => Self::init_test(args),

            "call" => Self::init_call(args),
            "ret" => Self::init_ret(args),
            "push" => Self::init_push(args),
            "pop" => Self::init_pop(args),

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
            Bne(_) => true,
            Bg(_) => true,
            Ble(_) => true,
            Call(_) => true,
            _ => false
        }
    }

    pub fn get_mark(&self) -> Option<&Mark<Address>> {
        use Instruction::*;
        match self {
            Jmp(mark) => Some(mark),
            Be(mark) => Some(mark),
            Bne(mark) => Some(mark),
            Bg(mark) => Some(mark),
            Ble(mark) => Some(mark),
            La(_, mark) => Some(mark),
            Lw(_, mark) => Some(mark),
            Lb(_, mark) => Some(mark),
            Lbu(_, mark) => Some(mark),
            Stw(mark, _) => Some(mark),
            Stb(mark, _) => Some(mark),
            Call(mark) => Some(mark),
            _ => None
        }
    }

    pub fn set_mark(&self, mark: Mark<Address>) -> Result<Instruction, LinkError> {
        use Instruction::*;
        match self {
            Jmp(_) => Ok(Jmp(mark)),
            Be(_) => Ok(Be(mark)),
            Bne(_) => Ok(Bne(mark)),
            Bg(_) => Ok(Bg(mark)),
            Ble(_) => Ok(Ble(mark)),
            La(target, _) => Ok(La(target, mark)),
            Lw(target, _) => Ok(Lw(target, mark)),
            Lb(target, _) => Ok(Lb(target, mark)),
            Lbu(target, _) => Ok(Lbu(target, mark)),
            Stw(_, source) => Ok(Stw(mark, source)),
            Stb(_, source) => Ok(Stb(mark, source)),
            Call(_) => Ok(Call(mark)),
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

    fn init_in(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::In(
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

    fn init_bne(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Bne(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Bne(Mark::Label(args[0].to_owned())))
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

    fn init_ble(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Ble(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Ble(Mark::Label(args[0].to_owned())))
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

    fn init_lbi(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Lbi(get_register(args[0])?, get_register(args[1])?))
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

    fn init_stwi(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Stwi(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_stbi(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Stbi(get_register(args[0])?, get_register(args[1])?))
    }

    fn init_inc(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        Ok(Instruction::Inc(
            get_register(args[0])?,
        ))
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

    fn init_and(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 3)?;

        Ok(Instruction::And(
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

    fn init_test(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 2)?;

        Ok(Instruction::Test(
            get_register(args[0])?, 
            get_register(args[1])?, 
        ))
    }

    fn init_call(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        match args[0].parse::<Address>() {
            Ok(ok) => Ok(Instruction::Call(Mark::Address(ok))),
            Err(err) => match err.kind() {
                IntErrorKind::Empty => return Err(ParseError::InvalidCommandArgumants),
                _ => Ok(Instruction::Call(Mark::Label(args[0].to_owned())))
            }
        }
    }

    fn init_ret(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 0)?;

        Ok(Instruction::Ret)
    }

    fn init_push(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        Ok(Instruction::Push(
            get_register(args[0])?
        ))
    }

    fn init_pop(args: &[&str]) -> Result<Instruction, ParseError> {
        check_args_len(args, 1)?;

        Ok(Instruction::Pop(
            get_register(args[0])?
        ))
    }

}

fn get_token_elements(token: &str) -> (&str, Vec<&str>) {
    let instr_args = token.split_once(INSTRUCTION_ARGUMENTS_SEPARATOR);

    match instr_args {
        Some(instr_args) => (
            instr_args.0, 
            parse_arguments(instr_args.1)
        ),
        None => (token, Vec::new())
    }
}


fn parse_arguments(input: &str) -> Vec<&str> {
    let re = regex::Regex::new(r#""[^"]*"|[^",\s]+"#).unwrap();

    re.find_iter(input)
        .map(|mat| mat.as_str())
        .map(|raw_arg| {
            match raw_arg.strip_suffix("\"") {
                None => return raw_arg,
                Some(stripped) => {
                    match stripped.strip_prefix("\"") {
                        None => stripped,
                        Some(arg) => arg
                    }
                }
            }
        }).collect()
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


#[derive(Clone)]
pub enum DataCommand {
    Byte(u8),
    Str(String),
    Vec(Mark<Address>)
}

impl FromStr for DataCommand {
    type Err = ParseError;

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let token_elements = get_token_elements(raw_command);

        let args = token_elements.1.as_slice();
        match token_elements.0 {
            "byte" => Self::init_byte(args),
            "char" => Self::init_char(args),
            "str" => Self::init_str(args),
            "vec" => Self::init_vec(args),

            "" => Err(ParseError::EmptyLineAsCommand),
            _ => Err(ParseError::NoSuchCommand(token_elements.0.to_owned()))
        }
    }
}

impl DataCommand {
    pub fn can_contain_label(&self) -> bool {
        self.is_data_referencing() || self.is_instruction_referencing()
    }

    pub fn is_data_referencing(&self) -> bool {
        match self {
            _ => false
        }
    }

    pub fn is_instruction_referencing(&self) -> bool {
        use DataCommand::*;
        match self {
            Vec(_) => true,
            _ => false
        }
    }

    pub fn get_mark(&self) -> Option<&Mark<Address>> {
        use DataCommand::*;
        match self {
            Vec(mark) => Some(mark),
            _ => None
        }
    }

    pub fn set_mark(&self, mark: Mark<Address>) -> Result<DataCommand, LinkError> {
        use DataCommand::*;
        match self {
            Vec(_) => Ok(Vec(mark)),
            _ => Err(LinkError::UnmarkableInstruction)
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

    fn init_vec(args: &[&str]) -> Result<DataCommand, ParseError> {
        check_args_len(args, 1)?;

        Ok(DataCommand::Vec(
            Mark::Label(args[0].to_owned())
        ))
    }

    pub fn get_size(&self) -> usize {
        use DataCommand::*;
        match self {
            Byte(_) => 1,
            Str(str) => str.len() + 1,
            Vec(_) => 32 / 8
        }
    }
}



#[derive(Clone)]
pub enum Mark<T> {
    Label(String),
    Address(T)
}

impl<T> Mark<T> {
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