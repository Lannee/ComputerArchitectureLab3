use super::GlobRegister;

use serde::Serialize;

#[derive(Serialize)]
pub enum Command {
    MovRegReg(GlobRegister, GlobRegister),
    MovRegVal(GlobRegister, i32),

    In(GlobRegister, GlobRegister),
    Out(GlobRegister, GlobRegister),
}