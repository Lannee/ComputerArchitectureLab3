use self::{clock::Clock, datapath::*, memory::Memory, ports::IOInterface, register::*};
use crate::{cpu::decoder::Decoder, errors::ExecutionError, input::machine_code::{Address, Data, Instruction, Instructions}, logger::Logger};

pub mod register;
pub mod datapath;
pub mod clock;
pub mod decoder;
pub mod memory;
pub mod ports;

const DATA_MEMORY_CAPACITY: usize = 0x1000;
const INSTRUCTION_MEMORY_CAPACITY: usize = 0x1000;

#[derive(Debug)]
pub struct CPU {
    pub datapath: DataPath,
    pub clock: Clock,
    pub ip: Register<Address>,
    pub instr_memory: Memory<Instruction>,

    pub io: IOInterface,
    int_req: bool,
    int_enabled: bool,

    __logger: Logger,
    __curr_instr: Option<Instruction>,
    __curr_instr_addr: Option<Address>,
}

impl CPU {
    pub fn new(instructions: Instructions, data: Data, logger: Logger) -> CPU {
        CPU {
            __logger: logger,
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
            int_req: false,
            int_enabled: true,

            __curr_instr: None,
            __curr_instr_addr: None,
        }
    }

    pub fn start(mut self) -> Result<(), ExecutionError> {
        let cpu_mut = unsafe { ::you_can::borrow_unchecked(&mut self) };


        let mut decoder = Decoder::new(cpu_mut);

        loop {
            let instruction = self.instr_memory.read(self.ip.value);
            self.__curr_instr = Some(instruction.clone());
            self.__curr_instr_addr = Some(self.ip.value);
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
        self.log_state()
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


impl CPU {
    fn log_state(&mut self) {
        self.__logger.write_cpu_state_log(
            self.__curr_instr_addr.unwrap(),
                self.__curr_instr.as_ref().unwrap(),
                self.clock.0,

                self.datapath.reg0.value,
                self.datapath.reg1.value,
                self.datapath.reg2.value,
                self.datapath.reg3.value,
                self.datapath.reg4.value,
                self.datapath.reg5.value,
                self.datapath.reg6.value,
                self.datapath.reg7.value,

                self.datapath.stack_p.value,
                (self.datapath.addr_reg.value as usize % self.datapath.memory.get_capacity()) as u32,

                self.datapath.alu.zero_flag,
                self.datapath.alu.neg_flag,
            )
    }

    fn log_int(&mut self) {
        self.__logger.write_cpu_int(
                &self.io.selected,
                self.io.data
            );
    } 
}