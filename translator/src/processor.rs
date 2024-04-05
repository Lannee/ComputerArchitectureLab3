pub mod register;
pub mod commands;

use register::*;

static PROCESSOR: ProcEntries = ProcEntries {
    registers: &[REG0, REG1],
};

pub struct ProcEntries<'a> {
    registers: &'a [GlobRegister]
}


impl<'a> ProcEntries<'a> {
    fn get_register(&self, reg_name: &String) -> Option<&Register<'static>> {
        self.registers
            .iter()
            .filter(|el| 
                el.has_name(reg_name)
            ).next()
    }
}