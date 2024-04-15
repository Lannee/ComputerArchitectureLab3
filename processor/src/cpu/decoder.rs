use crate::{cpu::{ALUOperation, Latch}, errors::ExecutionError, input::machine_code::Instruction};

use super::CPU;
use crate::{latch_reg_in, latch_reg_out_l, latch_reg_out_r, reg_out_latch};



pub struct Decoder<'a> {
    pub cu: &'static mut CPU,
    pub instruction: &'a Instruction
}


impl<'a> Decoder<'a> {
    pub fn execute_instruction(&mut self) -> Result<(), ExecutionError> {
        use Instruction::*;
        match self.instruction {
            Mov(target, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Movn(target, value) => {
                self.cu.datapath.latch(Latch::DecALUl(*value as u32));
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Out(target, value) => {

            },
            Jmp(address) => {
                self.select_ip_input(IpSelect::FromInstruction(*address as usize));
                self.cu.clock.tick();
            },
            La(target, address) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Add(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Sub(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Mul(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Mul);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Rem(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Rem);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
            },
            Nop => self.cu.clock.tick()
        };
        Ok(())
    }

    pub fn select_ip_input(&mut self, selection: IpSelect) {
        self.cu.ip.value = match selection {
                IpSelect::Inc => self.cu.ip.value + 1,
                IpSelect::DataPath => self.cu.datapath.alu.output as usize,
                IpSelect::FromInstruction(address) => address
            }
    }
}

#[macro_export]
macro_rules! latch_reg_out_l {
    ($index:expr, $datapath:expr) => {
        $datapath.latch(
            reg_out_latch!($index, $datapath).0
        )
    };
}

#[macro_export]
macro_rules! latch_reg_out_r {
    ($index:expr, $datapath:expr) => {
        $datapath.latch(
            reg_out_latch!($index, $datapath).1
        )
    };
}

#[macro_export]
macro_rules! reg_out_latch {
    ($index:expr, $datapath:expr) => {
        match $datapath.get_register_out_latch($index) {
            Some(latch) => latch,
            None => return Err(ExecutionError::InvalidRegisterIndexError)
        }
    };
}

#[macro_export]
macro_rules! latch_reg_in {
    ($index:expr, $datapath:expr) => {
        $datapath.latch(
            match $datapath.get_register_in_latch($index) {
                Some(latch) => latch,
                None => return Err(ExecutionError::InvalidRegisterIndexError)
            }
        )
    };
}


pub enum IpSelect {
    Inc,
    DataPath,
    FromInstruction(usize)
}