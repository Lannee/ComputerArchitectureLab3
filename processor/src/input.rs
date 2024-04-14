pub mod env_args;
pub mod machine_code;

use std::fs::File;
use serde::Deserialize;

use crate::errors::{InputError, MachineCodeError};

use self::machine_code::{Data, Instrictions, Instruction, RawMachineCode};


pub type IntSchedule = Vec<Interupt>;

const LOG_FILE_DFLT_NAME: &str = "logs/log";

pub fn get_src() -> Result<(Instrictions, Data), InputError> {
    let content = std::fs::read_to_string(get_src_path()?).map_err(|err| InputError::FileError(err))?;
    let raw_machine_code = serde_json::from_str::<RawMachineCode>(&content).map_err(|err| InputError::MachineCodeError(MachineCodeError::ParseError(err)))?;

    Ok((
        raw_machine_code.raw_instructions
            .into_iter()
            .map(|raw| raw.try_into().map_err(|err| InputError::MachineCodeError(err)))
            .collect::<Result<Instrictions, InputError>>()?,
        raw_machine_code.raw_data
    ))
}

pub fn get_log_file() -> Result<File, InputError> {
    let env = env_args::EnvArgs::get();

    match env.logs {
        None => File::create(file_name_as_json(LOG_FILE_DFLT_NAME)),
        Some(log_path) => File::open(log_path)
    }
    .map_err(|err| InputError::FileError(err))
}

fn get_src_path() -> Result<String, InputError> {
    let env = env_args::EnvArgs::get();
    match env.source_code {
        None => Err(InputError::NoInputFileError),
        Some(src) => Ok(src) 
    }
}

pub fn get_schedule() -> Result<IntSchedule, InputError> {
    let env = env_args::EnvArgs::get();
    match env.int_schedule {
        None => Err(InputError::NoInteruptionScheduleError),
        Some(path) => {
            let content = std::fs::read_to_string(path).map_err(|err| InputError::FileError(err))?;

            serde_json::from_str::<IntSchedule>(&content).map_err(|err| InputError::InteruptScheduleParseError(err))
        }
    }
}

fn file_name_as_json(file_name: &str) -> String {
    let mut name = match file_name.rsplit_once(".") {
        None => file_name.to_string(),
        Some(name) => name.0.to_string()
    };

    name.push_str(".json");
    name
}


#[derive(Deserialize)]
pub struct Interupt(usize, char);