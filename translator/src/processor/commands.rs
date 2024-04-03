

pub struct MachineCode {
    instructions: f64
}

pub enum Command {
    MovRegReg(Register32, Register32),
    MovRegVal(Register32, i32),

    In(Register32, Register32),
    Out(Register32, Register32),
}