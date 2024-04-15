use self::{clock::Clock, datapath::*, memory::Memory, register::*};
use crate::input::machine_code::{Data, Instruction, Instructions};

pub mod register;
pub mod datapath;
pub mod clock;
pub mod decoder;
pub mod memory;

const DATA_MEMORY_CAPACITY: usize = 1024;

pub struct CPU {
    pub datapath: DataPath,
    pub clock: Clock,
    pub ip: Register<usize>,
    pub instr_memory: Memory<Instruction>,
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
        
                addr_reg: Register::<usize>::new(),
        
                alu: ALU { left_input: 0, right_input: 0, output: 0 },
                memory: Memory::from_data_with_spec_size(data, DATA_MEMORY_CAPACITY)
            },
            clock: Clock(0),
            ip: Register::<usize>::new(),
            instr_memory: Memory::from_data_with_spec_size(instructions, DATA_MEMORY_CAPACITY),
        }
    }
}