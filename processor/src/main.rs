
use cpu::CPU;
use errors::ProcessorError;
use logger::Logger;
use out_device::Device;

mod cpu;
mod input;
mod errors;
mod out_device;
mod logger;

fn main() -> Result<(), ProcessorError> {
    let schedule = input::get_schedule().map_err(|err| ProcessorError::InputError(err))?;
    let (instructions, data) = input::get_src().map_err(|err| ProcessorError::InputError(err))?;
    let logger = Logger::new().map_err(|err| ProcessorError::InputError(err))?;

    let dev0 = Device::new(schedule);

    let mut cpu = CPU::new(instructions, data, logger);
    cpu.io.port0.connect_device(dev0);

    cpu.start().map_err(|err| ProcessorError::ExecutionError(err))?;

    Ok(())
}
