use std::path::Display;

use serde::Deserialize;

use crate::cpu::ports::PortSelect;


pub type Data = Vec<u8>;
pub type Address = u32;
type RegIndex = usize; 

pub type Instructions = Vec<Instruction>;

#[derive(Deserialize)]
pub struct MachineCode {
    pub instructions: Instructions,
    pub data: Data
}


#[derive(Deserialize, Debug, Clone)]
pub enum Instruction {
    Mov(RegIndex, RegIndex),
    Movn(RegIndex, i32),

    In(RegIndex, PortSelect),
    Out(PortSelect, RegIndex),
    Di,
    Ei,

    Jmp(Address),
    Be(Address),
    Bne(Address),
    Bg(Address),

    La(RegIndex, Address), 
    Lw(RegIndex, Address),
    Lb(RegIndex, Address),
    Lbi(RegIndex, RegIndex),
    Stw(Address, RegIndex),
    Stb(Address, RegIndex),
    Stwi(RegIndex, RegIndex),
    Stbi(RegIndex, RegIndex),

    Inc(RegIndex),
    Add(RegIndex, RegIndex, RegIndex),
    Sub(RegIndex, RegIndex, RegIndex),
    Mul(RegIndex, RegIndex, RegIndex),
    Rem(RegIndex, RegIndex, RegIndex),
    And(RegIndex, RegIndex, RegIndex),
    Cmp(RegIndex, RegIndex),
    Test(RegIndex, RegIndex),

    Call(Address),
    Ret,
    Push(RegIndex),
    Pop(RegIndex),

    Nop,
    Halt
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Nop
    }
}


impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Instruction::*;
        match self {
            Mov(tgt, src) => write!(f, "mov {tgt}, {src}"),
            Movn(tgt, int) => write!(f, "movn {tgt}, {int}"),

            In(tgt, port) => write!(f, "in {tgt}, {port}"),
            Out(port, src) => write!(f, "out {port}, {src}"),
            Di => write!(f, "di"),
            Ei => write!(f, "ei"),

            Jmp(addr) => write!(f, "jmp {addr}"),
            Be(addr) => write!(f, "be {addr}"),
            Bne(addr) => write!(f, "bne {addr}"),
            Bg(addr) => write!(f, "bg {addr}"),

            La(tgt, addr) => write!(f, "la {tgt}, {addr}"), 
            Lw(tgt, addr) => write!(f, "lw {tgt}, {addr}"),
            Lb(tgt, addr) => write!(f, "lb {tgt}, {addr}"),
            Lbi(tgt, src) => write!(f, "lbi {tgt}, {src}"),
            Stw(addr, src) => write!(f, "stw {addr}, {src}"),
            Stb(addr, src) => write!(f, "stb {addr}, {src}"),
            Stwi(tgt, src) => write!(f, "stwi {tgt}, {src}"),
            Stbi(tgt, src) => write!(f, "stbi {tgt}, {src}"),

            Inc(tgt) => write!(f, "inc {tgt}"),
            Add(tgt, src1, src2) => write!(f, "add {tgt}, {src1}, {src2}"),
            Sub(tgt, src1, src2) => write!(f, "sub {tgt}, {src1}, {src2}"),
            Mul(tgt, src1, src2) => write!(f, "mul {tgt}, {src1}, {src2}"),
            Rem(tgt, src1, src2) => write!(f, "rem {tgt}, {src1}, {src2}"),
            And(tgt, src1, src2) => write!(f, "and {tgt}, {src1}, {src2}"),
            Cmp(src1, src2) => write!(f, "cmp {src1}, {src2}"),
            Test(src1, src2) => write!(f, "test {src1}, {src2}"),

            Call(addr) => write!(f, "call {addr}"),
            Ret => write!(f, "ret"),
            Push(src) => write!(f, "push {src}"),
            Pop(tgt) => write!(f, "pop {tgt}"),

            Nop => write!(f, "nop"),
            Halt => write!(f, "halt")
        }
    }
}