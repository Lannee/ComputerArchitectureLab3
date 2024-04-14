use self::register::*;
use crate::new_register32;

pub mod register;
pub mod datapath;

pub static mut CPU: CPU = CPU {
    registers: &[
        new_register32!(),  // REG0
        new_register32!(),  // REG1
        new_register32!(),  // REG2
        new_register32!(),  // REG3
        new_register32!(),  // REG4
        new_register32!(),  // REG5
        new_register32!(),  // REG6
        new_register32!()   // REG7
    ],

};

pub struct CPU {
    pub registers: &'static [Register32]
}