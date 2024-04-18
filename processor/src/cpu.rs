use self::{clock::Clock, datapath::*, memory::Memory, ports::IOInterface, register::*};
use crate::{cpu::decoder::Decoder, errors::ExecutionError, input::machine_code::{Address, Data, Instruction, Instructions}};

pub mod register;
pub mod datapath;
pub mod clock;
pub mod decoder;
pub mod memory;
pub mod ports;

const DATA_MEMORY_CAPACITY: usize = 1024;
const INSTRUCTION_MEMORY_CAPACITY: usize = 1024;

#[derive(Debug)]
pub struct CPU {
    pub datapath: DataPath,
    pub clock: Clock,
    pub ip: Register<Address>,
    pub instr_memory: Memory<Instruction>,

    pub io: IOInterface,
    int: bool,
    int_req: bool,
    int_enabled: bool,
}

impl CPU {
    pub fn new(instructions: Instructions, data: Data) -> CPU {
        CPU {
            datapath: DataPath {
                reg0: Register32::new(),
                reg1: Register32::new(),
                reg2: Register32::new(),
                reg3: Register32::new(),
                reg4: Register32::new(),
                reg5: Register32::new(),
                reg6: Register32::new(),
                reg7: Register32::new(),
                
                stack_p: Register { value: 0 },
                addr_reg: Register::new(),
        
                alu: ALU::new(),
                memory: Memory::from_data_with_spec_size(data, DATA_MEMORY_CAPACITY),
            },
            clock: Clock(0),
            ip: Register::new(),
            instr_memory: Memory::from_data_with_spec_size(instructions, INSTRUCTION_MEMORY_CAPACITY),

            io: IOInterface::new(),
            int: false,
            int_req: false,
            int_enabled: true,
        }
    }

    pub fn start(mut self) -> Result<(), ExecutionError> {
        let cpu_mut = unsafe { ::you_can::borrow_unchecked(&mut self) };


        let mut decoder = Decoder::new(cpu_mut);

        loop {
            let instruction = self.instr_memory.read(self.ip.value);
            decoder.select_ip_input(decoder::IpSelect::Inc);

            if let Some(ProcSig::Halt) = decoder.execute_instruction(instruction)? {
                break;
            }
        }
        Ok(())
    }

    pub fn tick(&mut self) {
        self.clock.tick();
        self.datapath.tick();
        self.io.tick();

        self.int_req = self.io.int_req;
    }

    pub fn latch(&mut self, latch: CPULatch) {
        use CPULatch::*;
        match latch {
            IPIncDP => self.datapath.alu.right_input = self.ip.value,
            IODP => self.datapath.alu.output = self.io.data as u32,
            DPIO => {
                self.io.data = self.datapath.alu.output as u8;
                self.io.output();
            },
        }
    }
}


pub enum CPULatch {
    IPIncDP,
    IODP,
    DPIO,
}


pub enum ProcSig {
    Halt
}