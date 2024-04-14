use crate::input::machine_code::Instruction;

use super::CPU;




pub struct Decoder<'a> {
    pub cu: &'static mut CPU,
    pub instruction: &'a Instruction
}


impl<'a> Decoder<'a> {
    pub fn execute_instruction(&mut self) {
        use Instruction::*;
        match self.instruction {
            Mov(target, source) => {
                self.cu.clock.tick();
            },
            Movn(target, value) => {

            },
            Out(target, value) => {

            },
            Jmp(address) => {

            },
            La(target, address) => {

            },
            Add(target, source1, source2) => {

            },
            Sub(target, source1, source2) => {

            },
            Mul(target, source1, source2) => {

            },
            Rem(target, source1, source2) => {

            },
        }
    }
}