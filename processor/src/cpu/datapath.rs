use crate::input::machine_code::Instruction;
use crate::new_register32;

use super::Register32;

// pub static mut DATAPATH: DataPath = DataPath {
//     registers: &[
//         new_register32!(),  // REG0
//         new_register32!(),  // REG1
//         new_register32!(),  // REG2
//         new_register32!(),  // REG3
//         new_register32!(),  // REG4
//         new_register32!(),  // REG5
//         new_register32!(),  // REG6
//         new_register32!()   // REG7
//     ],
//     alu: ALU { left_input: 0, right_input: 0, output: 0 }
// };


pub struct DataPath {
    pub reg0: Register32, 
    pub reg1: Register32,
    pub reg2: Register32,
    pub reg3: Register32,
    pub reg4: Register32,
    pub reg5: Register32,
    pub reg6: Register32,
    pub reg7: Register32,

    pub alu: ALU,
}

impl DataPath {
    // pub fn get_register(&self, index: usize) -> Option<&Register32> {
    //     self.registers.get(index)
    // }

    pub fn latch(latch: Latch) {
        match latch {
            
        }
    }
}


pub enum Latch {
    LatchReg0,
    LatchReg1,
    LatchReg2,
    LatchReg3,
    LatchReg4,
    LatchReg5,
    LatchReg6,
    LatchReg7,
}


pub struct ALU {
    pub left_input: i32,
    pub right_input: i32,
    pub output: i32,

    // flags: 
}

impl ALU {
    pub fn execute_operation(&mut self, operation: ALUOperation) {
        use ALUOperation::*;
        self.output = match operation {
            Add => self.left_input + self.right_input,
            Sub => self.left_input - self.right_input,
            Mul => self.left_input * self.right_input,
            Rem => self.left_input % self.right_input,
            Cmp => self.left_input - self.right_input,
        };
    }
}


pub enum ALUOperation {
    Add,
    Sub,
    Mul,
    Rem,
    Cmp,

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