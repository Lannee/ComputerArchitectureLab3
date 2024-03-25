mod input;
mod translator;
mod errors;

use std::io::Write;

use errors::TranslationError;
use input::SourceCode;
use translator::parser::parse;

fn main() -> Result<(), TranslationError> {
    let code: SourceCode = input::get_src()
        .map_err(|err| TranslationError::InputError(err))?;

    let machine_code = parse(code)?;

    input::get_target_file()
        .map_err(|err| TranslationError::InputError(err))?
        .write_all(serde_json::to_string(&machine_code).unwrap().as_bytes())
        .unwrap();

    Ok(())
}