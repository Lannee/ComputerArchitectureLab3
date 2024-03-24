mod input;
mod translator;
mod errors;

use errors::TranslationError;
use input::SourceCode;

fn main() -> Result<(), TranslationError> {
    let code: SourceCode = input::get_src()
        .map_err(|err| TranslationError::InputError(err))?;




    Ok(())
}