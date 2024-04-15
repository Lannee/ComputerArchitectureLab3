use serde::Deserialize;

use crate::{cpu::{register::Register32, CPU}, errors::MachineCodeError};

// pub type RawInstrictions = Vec<RawInstriction>;
// pub type RawData = Data;
pub type Data = Vec<u8>;
type Address = u32;
type RegIndex = usize;

pub type Instructions = Vec<Instruction>;

#[derive(Deserialize)]
pub struct MachineCode {
    pub instructions: Instructions,
    pub data: Data
}


#[derive(Deserialize)]
pub enum Instruction {
    Mov(RegIndex, RegIndex),
    Movn(RegIndex, i32),

    Out(RegIndex, RegIndex),

    Jmp(Address),

    La(RegIndex, Address),

    Add(RegIndex, RegIndex, RegIndex),
    Sub(RegIndex, RegIndex, RegIndex),
    Mul(RegIndex, RegIndex, RegIndex),
    Rem(RegIndex, RegIndex, RegIndex),

    Nop
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Nop
    }
}

// impl TryInto<Instruction> for RawInstriction {
//     type Error = MachineCodeError;

//     fn try_into(self) -> Result<Instruction, Self::Error> {
//         fn get_register(index: usize) -> Result<&'static Register32, MachineCodeError> {
//             match unsafe {CPU.datapath.get_register(index)} {
//                 Some(reg) => Ok(reg),
//                 None => Err(MachineCodeError::InvalidRegisterIndex)
//             }
//         }

//         use RawInstriction::*;
//         match self {
//             Mov(target, source) => Ok(Instruction::Mov(get_register(target)?, get_register(source)?)),
//             Movn(target, value) => Ok(Instruction::Movn(get_register(target)?, value)),
//             Out(target, source) => Ok(Instruction::Out(get_register(target)?, get_register(source)?)),
//             Jmp(address) => Ok(Instruction::Jmp(address)),
//             La(target, address) => Ok(Instruction::La(get_register(target)?, address)),
//             Add(target, source1, source2) => Ok(Instruction::Add(get_register(target)?, get_register(source1)?, get_register(source2)?)),
//             Sub(target, source1, source2) => Ok(Instruction::Sub(get_register(target)?, get_register(source1)?, get_register(source2)?)),
//             Mul(target, source1, source2) => Ok(Instruction::Mul(get_register(target)?, get_register(source1)?, get_register(source2)?)),
//             Rem(target, source1, source2) => Ok(Instruction::Rem(get_register(target)?, get_register(source1)?, get_register(source2)?)),

//         }
//     }   
// }

// #[derive(Debug)]
// pub enum Instruction {
//     Mov(&'static Register32, &'static Register32),
//     Movn(&'static Register32, i32),

//     Out(&'static Register32, &'static Register32),

//     Jmp(Address),

//     La(&'static Register32, Address),

//     Add(&'static Register32, &'static Register32, &'static Register32),
//     Sub(&'static Register32, &'static Register32, &'static Register32),
//     Mul(&'static Register32, &'static Register32, &'static Register32),
//     Rem(&'static Register32, &'static Register32, &'static Register32),
// }