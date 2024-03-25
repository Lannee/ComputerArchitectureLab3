pub mod env_args;

use std::fs::File;

use crate::errors::TranslatorInputError;

pub type SourceCode = String;

pub fn get_src() -> Result<SourceCode, TranslatorInputError> {
    std::fs::read_to_string(
        get_src_path()?
    ).map_err(|err| TranslatorInputError::FileError(err))
}

pub fn get_target_file() -> Result<File, TranslatorInputError> {
    let env = env_args::EnvArgs::get();

    match env.out_file {
        None => File::create(file_name_as_json(&get_src_path()?)),
        Some(target_path) => File::open(target_path)
    }
    .map_err(|err| TranslatorInputError::FileError(err))
}

fn get_src_path() -> Result<String, TranslatorInputError> {
    let env = env_args::EnvArgs::get();
    match env.source_file {
        None => Err(TranslatorInputError::NoInputFile),
        Some(src) => Ok(src) 
    }
}

fn file_name_as_json(file_name: &String) -> String {
    let mut name = match file_name.rsplit_once(".") {
        None => file_name.clone(),
        Some(name) => name.0.to_string()
    };

    name.push_str(".json");
    name
}