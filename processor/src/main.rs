use cpu::CPU;
use errors::ProcessorError;

mod cpu;
mod input;
mod errors;

fn main() -> Result<(), ProcessorError> {
    let schedule = input::get_schedule().map_err(|err| ProcessorError::InputError(err))?;
    let (instructions, data) = input::get_src().map_err(|err| ProcessorError::InputError(err))?;
    
    let CPU = CPU::new(instructions, data);
    

    Ok(())
}
