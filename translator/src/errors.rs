use core::fmt;

pub enum TranslationError {
    InputError(TranslatorInputError),
    ParseError(ParseError),
    LinkError(LinkError)
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
            TranslationError::ParseError(err) => write!(f, "{err}"),
            TranslationError::LinkError(err) => write!(f, "{err}"),
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


pub enum ParseError {
    NoSuchCommand(String),
    EmptyLineAsCommand,
    InvalidCommandArgumants,
    InvalidAmountOfCommandArguments(Vec<String>, usize, usize),

    InstructionInDataSection,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;

        match self {
            NoSuchCommand(command) => write!(f, "Unknown command \"{}\"", command),
            EmptyLineAsCommand => write!(f, "Empty line is trying to be treated as command"),
            InvalidCommandArgumants => write!(f, "Invalid command arguments"),
            InvalidAmountOfCommandArguments(args, expected, found) => write!(f, "Invalid number of command arguments: \"{}\".\nExpected {expected}, but found {found}", args.join(", ")),

            InstructionInDataSection => write!(f, "Found inctruction in data section"),
        }
    }
}


pub enum LinkError {
    UnmarkableInstruction,
    UndefinedDataLabel(String),
    UndefinedInstructionLabel(String),
    UnresolvedInstruction(String),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LinkError::*;
        match self {
            UnmarkableInstruction => write!(f, "instruction cannot be marked"),
            UndefinedDataLabel(label) => write!(f, "cannot define data label \"{label}\""),
            UndefinedInstructionLabel(label) => write!(f, "cannot define instruction label \"{label}\""),
            UnresolvedInstruction(label) => write!(f, "cannot resolve label \"{label}\""),
        }
    }
}