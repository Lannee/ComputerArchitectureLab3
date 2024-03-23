use crate::architecture::register::Register;

pub enum Command {
    MovRegReg(Register, Register),
    MovRegVal(Register, i32),

    In(Register, Register),
    Out(Register, Register),
}