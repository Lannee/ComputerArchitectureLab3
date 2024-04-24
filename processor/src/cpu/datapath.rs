use crate::input::machine_code::Address;

use super::memory::ByteMemory;
use super::{Register, Register32};

#[derive(Debug)]
pub struct DataPath {
    pub reg0: Register32, 
    pub reg1: Register32,
    pub reg2: Register32,
    pub reg3: Register32,
    pub reg4: Register32,
    pub reg5: Register32,
    pub reg6: Register32,
    pub reg7: Register32,

    pub stack_p: Register<Address>,
    pub addr_reg: Register<Address>,

    pub alu: ALU,
    pub memory: ByteMemory,
}

impl DataPath {
    pub fn get_register_alu_l_select(&self, index: usize) -> Option<ALUlSelect> {
        use ALUlSelect::*;
        match index {
            0 => Some(R0),
            1 => Some(R1),
            2 => Some(R2),
            3 => Some(R3),
            4 => Some(R4),
            5 => Some(R5),
            6 => Some(R6),
            7 => Some(R7),
            _ => None
        }
    }

    pub fn get_register_alu_r_select(&self, index: usize) -> Option<ALUrSelect> {
        use ALUrSelect::*;
        match index {
            0 => Some(R0),
            1 => Some(R1),
            2 => Some(R2),
            3 => Some(R3),
            4 => Some(R4),
            5 => Some(R5),
            6 => Some(R6),
            7 => Some(R7),
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

    pub fn sel_alu_l(&mut self, select: ALUlSelect) {
        use ALUlSelect::*;
        self.alu.left_input = match select {
            R0 => self.reg0.value,
            R1 => self.reg1.value,
            R2 => self.reg2.value,
            R3 => self.reg3.value,
            R4 => self.reg4.value,
            R5 => self.reg5.value,
            R6 => self.reg6.value,
            R7 => self.reg7.value,
            SP => self.stack_p.value,
        
            Dcr(value) => value,
            IntVec(addr) => addr,
            IPIncDP(value) => value,

            Zero => 0
        }
    }

    pub fn sel_alu_r(&mut self, select: ALUrSelect) {
        use ALUrSelect::*;
        self.alu.right_input = match select {
            R0 => self.reg0.value,
            R1 => self.reg1.value,
            R2 => self.reg2.value,
            R3 => self.reg3.value,
            R4 => self.reg4.value,
            R5 => self.reg5.value,
            R6 => self.reg6.value,
            R7 => self.reg7.value,

            Zero => 0
        }
    }

    pub fn latch(&mut self, latch: Latch) {
        use Latch::*;
        match latch {
            ALUoR0 => self.reg0.value = self.alu.output,
            ALUoR1 => self.reg1.value = self.alu.output,
            ALUoR2 => self.reg2.value = self.alu.output,
            ALUoR3 => self.reg3.value = self.alu.output,
            ALUoR4 => self.reg4.value = self.alu.output,
            ALUoR5 => self.reg5.value = self.alu.output,
            ALUoR6 => self.reg6.value = self.alu.output,
            ALUoR7 => self.reg7.value = self.alu.output,
            ALUoSP => self.stack_p.value = self.alu.output,

            AddrR => self.addr_reg.value = self.alu.output,
            WriteB => self.memory.write(self.addr_reg.value, self.alu.output as u8),
            WriteW => self.memory.write_w(self.addr_reg.value, self.alu.output),
            ReadB => self.alu.output = *self.memory.read(self.addr_reg.value) as u32,
            ReadW => self.alu.output = self.memory.read_w(self.addr_reg.value),
        }
    }


    pub fn tick(&mut self) {
        self.sel_alu_l(ALUlSelect::Zero);
        self.sel_alu_r(ALUrSelect::Zero);
    }
}


pub enum ALUlSelect {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    SP,

    Dcr(u32),
    IntVec(Address),
    IPIncDP(u32),

    Zero
}

pub enum ALUrSelect {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,

    Zero
}

pub enum Latch {
    ALUoR0,
    ALUoR1,
    ALUoR2,
    ALUoR3,
    ALUoR4,
    ALUoR5,
    ALUoR6,
    ALUoR7,
    ALUoSP,

    AddrR,
    WriteB,
    WriteW,
    ReadB,
    ReadW,
}

#[derive(Debug)]
pub struct ALU {
    pub left_input: u32,
    pub right_input: u32,
    pub output: u32,

    pub zero_flag: bool,
    pub neg_flag: bool, 
}

impl ALU {
    pub fn new() -> ALU {
        let mut alu = ALU { 
            left_input: 0, 
            right_input: 0, 
            output: 0,

            zero_flag: true,
            neg_flag: false,
        };

        alu.set_flags();
        alu
    }

    pub fn execute_operation(&mut self, operation: ALUOperation) {
        use ALUOperation::*;
        self.output = match operation {
            Add => self.left_input.overflowing_add(self.right_input).0,
            Sub => self.left_input.overflowing_sub(self.right_input).0,
            Mul => self.left_input.overflowing_mul(self.right_input).0,
            Rem => self.left_input.overflowing_rem(self.right_input).0,
            And => self.left_input & self.right_input,
            Inc => self.left_input.overflowing_add(1).0,
        };

        self.set_flags();
    }

    fn set_flags(&mut self) {
        self.zero_flag = self.output == 0;
        self.neg_flag = is_sign_bit_set(self.output);
    }
}


pub enum ALUOperation {
    Add,
    Sub,
    Mul,
    Rem,
    And,
    Inc,
}


fn is_sign_bit_set(value: u32) -> bool {
    value & (1 << u32::BITS-1) != 0
}