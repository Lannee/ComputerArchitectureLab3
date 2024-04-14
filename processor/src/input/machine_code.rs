use serde::Deserialize;

use crate::{architecture::register::Register32, errors::MachineCodeError};

pub type RawInstrictions = Vec<RawInstriction>;
// pub type RawData = Data;
pub type Data = Vec<u8>;
type Address = u32;
type RegIndex = usize;

pub type Instrictions = Vec<Instruction>;

#[derive(Deserialize)]
pub struct RawMachineCode {
    pub raw_instructions: RawInstrictions,
    pub raw_data: Data
}


#[derive(Deserialize)]
pub enum RawInstriction {
    Mov(RegIndex, RegIndex),
    Movn(RegIndex, RegIndex),

    In(RegIndex, RegIndex),
    Out(RegIndex, RegIndex),

    Jmp(Address),

    La(RegIndex, Address)
}

impl TryInto<Instruction> for RawInstriction {
    type Error = MachineCodeError;

    fn try_into(self) -> Result<Instruction, Self::Error> {
        todo!()
    }   
}


pub enum Instruction {
    Mov(&'static Register32, &'static Register32),
    Movn(&'static Register32, i32),

    In(&'static Register32, &'static Register32),
    Out(&'static Register32, &'static Register32),

    Jmp(Address),

    La(&'static Register32, Address)
}