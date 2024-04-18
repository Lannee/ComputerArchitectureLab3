use std::{cell::RefCell, rc::Rc};

use cpu::{ports::{self, IOInterface, Port}, CPU};
use errors::ProcessorError;
use out_device::Device;

mod cpu;
mod input;
mod errors;
mod out_device;

fn main() -> Result<(), ProcessorError> {
    let schedule = input::get_schedule().map_err(|err| ProcessorError::InputError(err))?;
    let (instructions, data) = input::get_src().map_err(|err| ProcessorError::InputError(err))?;

    let dev0 = Device::new(schedule);

    let mut CPU = CPU::new(instructions, data);
    CPU.io.port0.connect_device(dev0);

    CPU.start().map_err(|err| ProcessorError::ExecutionError(err))?;

    Ok(())
}
