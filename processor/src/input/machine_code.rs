use serde::Deserialize;

use crate::cpu::ports::PortSelect;


pub type Data = Vec<u8>;
type Address = u32;
type RegIndex = usize; 
type PortIndex = usize;

pub type Instructions = Vec<Instruction>;

#[derive(Deserialize)]
pub struct MachineCode {
    pub instructions: Instructions,
    pub data: Data
}


#[derive(Deserialize, Debug)]
pub enum Instruction {
    Mov(RegIndex, RegIndex),
    Movn(RegIndex, i32),

    Out(PortSelect, RegIndex),

    Jmp(Address),
    Be(Address),
    Bg(Address),

    La(RegIndex, Address), 
    Lw(RegIndex, Address),
    Lb(RegIndex, Address),
    Lbi(RegIndex, RegIndex),
    Lbu(RegIndex, Address),
    Lbui(RegIndex, RegIndex),
    Stw(Address, RegIndex),
    Stb(Address, RegIndex),

    Inc(RegIndex),
    Add(RegIndex, RegIndex, RegIndex),
    Sub(RegIndex, RegIndex, RegIndex),
    Mul(RegIndex, RegIndex, RegIndex),
    Rem(RegIndex, RegIndex, RegIndex),
    And(RegIndex, RegIndex, RegIndex),
    Cmp(RegIndex, RegIndex),
    Test(RegIndex, RegIndex),

    Nop,
    Halt
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Nop
    }
}