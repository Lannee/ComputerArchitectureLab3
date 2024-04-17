use std::fmt::Alignment;

use crate::{cpu::{ALUOperation, Latch}, errors::ExecutionError, input::machine_code::{Instruction, Instructions}};

use super::{ProcSig, CPU};
use crate::{latch_reg_in, latch_reg_out_l, latch_reg_out_r, reg_out_latch};



pub struct Decoder<'a> {
    pub cu: &'a mut CPU<'a>,
}


impl<'a> Decoder<'a> {
    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Result<Option<ProcSig>, ExecutionError> {
        // println!("{instruction:?}");
        use Instruction::*;
        match instruction {
            Mov(target, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Movn(target, value) => {
                self.cu.datapath.latch(Latch::DecALUl(*value as u32));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Out(port, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.io.select_port(port.clone());
                self.cu.io.send_data(self.cu.datapath.alu.output as u8);
                self.cu.tick();
            },
            Jmp(address) => {
                self.select_ip_input(IpSelect::FromInstruction(*address as usize));
                self.cu.tick();
            },
            Be(address) => {
                if self.cu.datapath.alu.zero_flag {
                    self.select_ip_input(IpSelect::FromInstruction(*address as usize));
                }
                self.cu.tick();
            },
            Bg(address) => {
                if !self.cu.datapath.alu.zero_flag && !self.cu.datapath.alu.neg_flag {
                    self.select_ip_input(IpSelect::FromInstruction(*address as usize));
                }
                self.cu.tick();
            },
            La(target, address) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.clock.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
                // println!("{:?}", self.cu.datapath.reg3);
            },
            Inc(target) => {
                latch_reg_out_l!(*target, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.datapath.alu.execute_operation(ALUOperation::Inc);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
                // println!("{:?}", self.cu.datapath.reg3);
            },
            Add(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Sub(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Mul(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Mul);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Rem(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Rem);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            And(target, source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::And);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Cmp(source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
                self.cu.tick();
            },
            Test(source1, source2) => {
                latch_reg_out_l!(*source1, self.cu.datapath);
                latch_reg_out_r!(*source2, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::And);
                // println!("out = {:?}", self.cu.datapath.alu);
                self.cu.tick();
            },
            Lw(target, address) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ReadW);
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Lb(target, address) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ReadB);
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Lbi(target, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ReadB);
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
                // println!("{:?}", self.cu.datapath.reg4);
            },
            Lbu(target, address) => {

            },
            Lbui(target, source) => {

            },
            Stw(target, address) => {

            },
            Stb(target, address) => {

            },
            Nop => self.cu.tick(),
            Halt => {
                self.cu.tick();
                return Ok(Some(ProcSig::Halt))
            },
        };
        Ok(None)
    }

    pub fn select_ip_input(&mut self, selection: IpSelect) {
        self.cu.ip.value = match selection {
                IpSelect::Inc => self.cu.ip.value + 1,
                IpSelect::DataPath => self.cu.datapath.alu.output as usize,
                IpSelect::FromInstruction(address) => address
            }
    }

    pub fn new(cu: &'a mut CPU<'a>) -> Decoder<'a> {
        Decoder {
            cu
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