use std::{fs::File, io::Write};

use crate::{cpu::{ports::PortSelect, CPU}, errors::InputError, input::{get_log_file, machine_code::{Address, Instruction}}};


#[derive(Debug)]
pub struct Logger {
    log_file: File
}

impl Logger {
    pub fn new() -> Result<Self, InputError> {
        Ok (
            Self {
                log_file: get_log_file()?
            }
        )
    }

    pub fn write_cpu_state_log(
        &mut self, 
        instr_addr: Address,
        instr: &Instruction,
        tick: usize,

        reg0: u32,
        reg1: u32,
        reg2: u32,
        reg3: u32,
        reg4: u32,
        reg5: u32,
        reg6: u32,
        reg7: u32,
        stack_p: u32,
        addr_reg: u32,
        
        zero_f: bool,
        neg_f: bool,
    ) {
        self.log_file.write(format!(
            "DEBUG: | {instr_addr:#x} - {instr} | tick : {tick:#x} -> r0 : {reg0:#x} | r1 : {reg1:#x} | r2 : {reg2:#x} | r3 : {reg3:#x} | r4 : {reg4:#x} | r5 : {reg5:#x} | r6 : {reg6:#x} | r7 : {reg7:#x} | sp : {stack_p:#x} | raddr : {addr_reg:#x} | zf : {} | nf : {} |\n",
            if zero_f {1} else {0},
            if neg_f {1} else {0}
        ).as_bytes())
            .expect("Cannot write to log file");
    } 

    pub fn write_cpu_int(
        &mut self,
        selected_port: &PortSelect,
        data_on_bus: u8,
    ) {
        self.log_file.write(format!(
            "INTERUPT: port : {} | on bus : {data_on_bus:#x}\n",
            selected_port.get_port_id(),
        ).as_bytes())
            .expect("Cannot write to log file");
    }
}