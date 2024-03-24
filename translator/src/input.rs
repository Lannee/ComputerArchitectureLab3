pub mod env_args;

use crate::errors::TranslatorInputError;

pub fn get_src() -> Result<SourceCode, TranslatorInputError> {
    let env = env_args::EnvArgs::get();
    match env.source_file {
        None => Err(TranslatorInputError::NoInputFile),
        Some(src) => {
            Ok(std::fs::read_to_string(src)
                .map_err(|err| TranslatorInputError::FileError(err))?)
        }  
    }
}


pub type SourceCode = String;