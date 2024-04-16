use crate::input::machine_code::Instruction;
use crate::new_register32;

use super::memory::{ByteMemory, Memory};
use super::{Register, Register32};


pub struct DataPath {
    pub reg0: Register32, 
    pub reg1: Register32,
    pub reg2: Register32,
    pub reg3: Register32,
    pub reg4: Register32,
    pub reg5: Register32,
    pub reg6: Register32,
    pub reg7: Register32,

    pub addr_reg: Register<usize>,

    pub alu: ALU,
    pub memory: ByteMemory,
}

impl DataPath {
    pub fn get_register_out_latch(&self, index: usize) -> Option<(Latch, Latch)> {
        use Latch::*;
        match index {
            0 => Some((R0ALUl, R0ALUr)),
            1 => Some((R1ALUl, R1ALUr)),
            2 => Some((R2ALUl, R2ALUr)),
            3 => Some((R3ALUl, R3ALUr)),
            4 => Some((R4ALUl, R4ALUr)),
            5 => Some((R5ALUl, R5ALUr)),
            6 => Some((R6ALUl, R6ALUr)),
            7 => Some((R7ALUl, R7ALUr)),
            _ => None
        }
    }

    pub fn get_register_in_latch(&self, index: usize) -> Option<Latch> {
        use Latch::*;
        match index {
            0 => Some(ALUoR0),
            1 => Some(ALUoR1),
            2 => Some(ALUoR2),
            3 => Some(ALUoR3),
            4 => Some(ALUoR4),
            5 => Some(ALUoR5),
            6 => Some(ALUoR6),
            7 => Some(ALUoR7),
            _ => None
        }
    }

    pub fn latch(&mut self, latch: Latch) {
        use Latch::*;
        match latch {
            R0ALUl => self.alu.left_input = self.reg0.value,
            R1ALUl => self.alu.left_input = self.reg1.value,
            R2ALUl => self.alu.left_input = self.reg2.value,
            R3ALUl => self.alu.left_input = self.reg3.value,
            R4ALUl => self.alu.left_input = self.reg4.value,
            R5ALUl => self.alu.left_input = self.reg5.value,
            R6ALUl => self.alu.left_input = self.reg6.value,
            R7ALUl => self.alu.left_input = self.reg7.value,
            R0ALUr => self.alu.right_input = self.reg0.value,
            R1ALUr => self.alu.right_input = self.reg1.value,
            R2ALUr => self.alu.right_input = self.reg2.value,
            R3ALUr => self.alu.right_input = self.reg3.value,
            R4ALUr => self.alu.right_input = self.reg4.value,
            R5ALUr => self.alu.right_input = self.reg5.value,
            R6ALUr => self.alu.right_input = self.reg6.value,
            R7ALUr => self.alu.right_input = self.reg7.value,
            ALUoR0 => self.reg0.value = self.alu.output,
            ALUoR1 => self.reg1.value = self.alu.output,
            ALUoR2 => self.reg2.value = self.alu.output,
            ALUoR3 => self.reg3.value = self.alu.output,
            ALUoR4 => self.reg4.value = self.alu.output,
            ALUoR5 => self.reg5.value = self.alu.output,
            ALUoR6 => self.reg6.value = self.alu.output,
            ALUoR7 => self.reg7.value = self.alu.output,
            
            DecALUl(value) => self.alu.left_input = value,
            AddrR => self.addr_reg.value = self.alu.output as usize,
            WriteB => self.memory.write(self.addr_reg.value, self.alu.output as u8),
            WriteW => self.memory.write_w(self.addr_reg.value, self.alu.output),
            ReadB => self.alu.output = *self.memory.read(self.addr_reg.value) as u32,
            ReadW => self.alu.output = self.memory.read_w(self.addr_reg.value),
        }
    }


    pub fn tick(&mut self) {
        self.alu.tick();
    }
}


pub enum Latch {
    R0ALUl,
    R1ALUl,
    R2ALUl,
    R3ALUl,
    R4ALUl,
    R5ALUl,
    R6ALUl,
    R7ALUl,
    R0ALUr,
    R1ALUr,
    R2ALUr,
    R3ALUr,
    R4ALUr,
    R5ALUr,
    R6ALUr,
    R7ALUr,
    ALUoR0,
    ALUoR1,
    ALUoR2,
    ALUoR3,
    ALUoR4,
    ALUoR5,
    ALUoR6,
    ALUoR7,

    DecALUl(u32),
    AddrR,
    WriteB,
    WriteW,
    ReadB,
    ReadW,
}


pub struct ALU {
    pub left_input: u32,
    pub right_input: u32,
    pub output: u32,

    pub zero_flag: bool, 
}

impl ALU {
    pub fn new() -> ALU {
        let mut alu = ALU { 
            left_input: 0, 
            right_input: 0, 
            output: 0,

            zero_flag: false,
        };

        alu.set_flags();
        alu
    }

    pub fn execute_operation(&mut self, operation: ALUOperation) {
        use ALUOperation::*;
        self.output = match operation {
            Add => self.left_input + self.right_input,
            Sub => self.left_input - self.right_input,
            Mul => self.left_input * self.right_input,
            Rem => self.left_input % self.right_input,
        };

        self.set_flags();
    }

    fn set_flags(&mut self) {
        self.zero_flag = self.output == 0;
    }

    fn tick(&mut self) {
        self.left_input = u32::default();
        self.right_input = u32::default();
    }
}


pub enum ALUOperation {
    Add,
    Sub,
    Mul,
    Rem,
}

impl ALUOperation {
    fn from(instruction: Instruction) -> Option<ALUOperation> {
        use Instruction::*;
        match instruction {
            Add(_, _, _) => Some(Self::Add),
            Sub(_, _, _) => Some(Self::Sub),
            Mul(_, _, _) => Some(Self::Mul),
            Rem(_, _, _) => Some(Self::Rem),

            _ => None
        }
    }
}

pub enum ALUFlag {
    Zero,

}