pub mod register;
pub mod commands;

use register::*;

pub static PROCESSOR: ProcEntries = ProcEntries {
    registers: &[REG0, REG1],
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