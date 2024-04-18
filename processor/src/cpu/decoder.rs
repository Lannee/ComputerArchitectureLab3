use std::mem;

use crate::{cpu::{ALUOperation, CPULatch, Latch}, errors::ExecutionError, input::machine_code::{Address, Instruction}};

use super::{ProcSig, CPU};
use crate::{latch_reg_in, latch_reg_out_l, latch_reg_out_r, reg_out_latch};



pub struct Decoder<'a> {
    pub cu: &'a mut CPU,
}


impl<'a> Decoder<'a> {
    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Result<Option<ProcSig>, ExecutionError> {
        // println!("{instruction:?}");
        use Instruction::*;
        match instruction {
            Mov(target, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
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
            In(target, port) => {
                self.cu.io.select_port(port.clone());
                self.cu.tick();
                self.cu.latch(CPULatch::IODP);
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.io.data_rx();
                self.cu.tick();
            },
            Out(port, source) => {
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.io.select_port(port.clone());
                self.cu.latch(CPULatch::DPIO);
                // self.cu.io.send_data();
                self.cu.tick();
            },
            Di => {
                self.cu.int_enabled = false;
                self.cu.tick();
            },
            Ei => {
                self.cu.int_enabled = true;
                self.cu.tick();
            },
            Jmp(address) => {
                self.select_ip_input(IpSelect::FromInstruction(*address));
                self.cu.tick();
            },
            Be(address) => {
                if self.cu.datapath.alu.zero_flag {
                    self.select_ip_input(IpSelect::FromInstruction(*address));
                }
                self.cu.tick();
            },
            Bne(address) => {
                if !self.cu.datapath.alu.zero_flag {
                    self.select_ip_input(IpSelect::FromInstruction(*address));
                }
                self.cu.tick();
            },
            Bg(address) => {
                if !self.cu.datapath.alu.zero_flag && !self.cu.datapath.alu.neg_flag {
                    self.select_ip_input(IpSelect::FromInstruction(*address));
                }
                self.cu.tick();
            },
            La(target, address) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
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
            Stw(address, source) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::WriteW);
                self.cu.tick();
            },
            Stb(address, source) => {
                self.cu.datapath.latch(Latch::DecALUl(*address));
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::WriteB);
                self.cu.tick();
            },
            Stwi(target, source) => {
                latch_reg_out_l!(*target, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::WriteW);
                self.cu.tick();
            },
            Stbi(target, source) => {
                latch_reg_out_l!(*target, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::WriteB);
                self.cu.tick();
            },
            Call(address) => {
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.right_input = mem::size_of::<Address>() as u32;
                self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ALUoSP);
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.latch(CPULatch::IPIncDP);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.memory.write_w(self.cu.datapath.addr_reg.value, self.cu.datapath.alu.output);
                self.select_ip_input(IpSelect::FromInstruction(*address));
                self.cu.tick();
            },  
            Ret => {
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.right_input = mem::size_of::<Address>() as u32;
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ALUoSP);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ReadW);
                self.cu.tick();
                self.select_ip_input(IpSelect::DataPath);
                self.cu.tick();
            },
            Push(source) => {
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.right_input = mem::size_of::<Address>() as u32;
                self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ALUoSP);
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                latch_reg_out_l!(*source, self.cu.datapath);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.memory.write_w(self.cu.datapath.addr_reg.value, self.cu.datapath.alu.output);
                self.cu.tick();
            },
            Pop(target) => {
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::AddrR);
                self.cu.tick();
                self.cu.datapath.latch(Latch::SPALUl);
                self.cu.datapath.alu.right_input = mem::size_of::<Address>() as u32;
                self.cu.datapath.alu.execute_operation(ALUOperation::Add);
                self.cu.tick();
                self.cu.datapath.latch(Latch::ALUoSP);
                self.cu.tick();
                self.cu.datapath.memory.read_w(self.cu.datapath.addr_reg.value);
                self.cu.tick();
                latch_reg_in!(*target, self.cu.datapath);
                self.cu.tick();
            },
            Nop => self.cu.tick(),
            Halt => {
                self.cu.tick();
                return Ok(Some(ProcSig::Halt))
            },
        };

        if self.cu.int_req & self.cu.int_enabled & !self.cu.int {
            let addr = self.cu.io.selected.clone() as usize * mem::size_of::<Address>();
            self.cu.datapath.latch(Latch::SPALUl);
            self.cu.datapath.alu.right_input = 4;
            self.cu.datapath.alu.execute_operation(ALUOperation::Sub);
            self.cu.tick();
            self.cu.datapath.latch(Latch::ALUoSP);
            self.cu.datapath.latch(Latch::AddrR);
            self.cu.tick();
            self.cu.latch(CPULatch::IPIncDP);
            self.cu.datapath.alu.execute_operation(ALUOperation::Add);
            self.cu.tick();
            self.cu.datapath.latch(Latch::WriteW);
            // self.cu.datapath.memory.write_w(self.cu.datapath.addr_reg.value, self.cu.datapath.alu.output);
            self.cu.tick();
            self.cu.datapath.latch(Latch::IntVecALUl(addr as u32));
            self.cu.datapath.alu.execute_operation(ALUOperation::Add);
            self.cu.tick();
            self.cu.datapath.latch(Latch::AddrR);
            self.cu.tick();
            self.cu.datapath.latch(Latch::ReadW);
            self.cu.tick();
            self.select_ip_input(IpSelect::DataPath);
            self.cu.tick();
        }
        Ok(None)
    }

    pub fn select_ip_input(&mut self, selection: IpSelect) {
        self.cu.ip.value = match selection {
                IpSelect::Inc => self.cu.ip.value.overflowing_add(1).0,
                IpSelect::DataPath => self.cu.datapath.alu.output,
                IpSelect::FromInstruction(address) => address
            }
    }

    pub fn new(cu: &'a mut CPU) -> Decoder<'a> {
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
    FromInstruction(Address)
}