mod env_args;
mod translator;
mod errors;

use errors::TranslationError;
use errors::TranslatorInputError;

fn main() -> Result<(), TranslationError> {
    let env = env_args::EnvArgs::get();
    match env.source_file {
        None => return Err(TranslationError::InputError(TranslatorInputError::NoInputFile)),
        Some(file) => todo!()
    }

    Ok(())
}