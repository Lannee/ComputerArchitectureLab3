pub mod register;
pub mod commands;

use register::*;

pub static PROCESSOR: ProcEntries = ProcEntries {
    registers: &[REG0, REG1, REG2, REG3, REG4, REG5, REG6, REG7],
};

pub struct ProcEntries<'a> {
    registers: &'a [GlobRegister]
}


impl<'a> ProcEntries<'a> {
    pub fn get_register(&self, reg_name: &str) -> Option<&Register<'static>> {
        self.registers
            .iter()
            .filter(|el| 
                el.has_name(reg_name)
            ).next()
    }
}