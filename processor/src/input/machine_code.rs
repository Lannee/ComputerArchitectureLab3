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
    Be(Address),

    La(RegIndex, Address),

    Add(RegIndex, RegIndex, RegIndex),
    Sub(RegIndex, RegIndex, RegIndex),
    Mul(RegIndex, RegIndex, RegIndex),
    Rem(RegIndex, RegIndex, RegIndex),
    Cmp(RegIndex, RegIndex),

    Lw(RegIndex, Address),
    Lb(RegIndex, Address),
    Lbu(RegIndex, Address),
    St(Address, RegIndex),

    Nop,
    Halt
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Nop
    }
}