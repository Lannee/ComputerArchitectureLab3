use self::{datapath::*, register::*, clock::Clock};
use crate::new_register32;

pub mod register;
pub mod datapath;
pub mod clock;
pub mod decoder;

pub static mut CPU: CPU = CPU {
    datapath: DataPath {
        reg0: new_register32!(),
        reg1: new_register32!(),
        reg2: new_register32!(),
        reg3: new_register32!(),
        reg4: new_register32!(),
        reg5: new_register32!(),
        reg6: new_register32!(),
        reg7: new_register32!(),

        alu: ALU { left_input: 0, right_input: 0, output: 0 }
    },
    clock: Clock(0),
};

pub struct CPU {
    pub datapath: DataPath,
    pub clock: Clock,
}

impl CPU {

}