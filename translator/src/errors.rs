use core::fmt;

pub enum TranslationError {
    InputError(TranslatorInputError),
}

impl fmt::Debug for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationError::InputError(err) => write!(f, "{err}"),
        }
    }    
}







const INPUT_FORMAT: &str = "
    cargo run -- <SOURCE_FILE> <TARGET_FILE>
    
    where SOURCE_FILE - file with source code
          TARGET_FILE - file to store out machine code";   


pub enum TranslatorInputError {
    NoInputFile,
    FileError(std::io::Error),
}

impl fmt::Display for TranslatorInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslatorInputError::NoInputFile => write!(f, "No input file. The following input format is required: {INPUT_FORMAT}"),
            TranslatorInputError::FileError(err) => write!(f, "File error: {err}"),
        }
    }
}