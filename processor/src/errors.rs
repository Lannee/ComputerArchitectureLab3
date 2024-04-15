use core::fmt;

pub enum ProcessorError {
    InputError(InputError),
    ExecutionError(ExecutionError),
}

pub enum ExecutionError {
    InvalidRegisterIndexError,
}


pub enum InputError {
    NoInputFileError,
    NoInteruptionScheduleError,
    InteruptScheduleParseError(serde_json::Error),
    MachineCodeError(MachineCodeError),
    FileError(std::io::Error)
}

pub enum MachineCodeError {
    ParseError(serde_json::Error),
    RawInstructionError,
    InvalidRegisterIndex,
}

impl fmt::Debug for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProcessorError::*;
        match self {
            InputError(err) => write!(f, "Input error: \n{err}"),
            ExecutionError(err) => write!(f, ""),
        }
    }    
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InputError::*;
        match self {
            NoInputFileError => write!(f, "No code to execute faund"),
            NoInteruptionScheduleError => write!(f, "No interuption schedule faund"),
            InteruptScheduleParseError(err) => write!(f, "Interuption schedule parse error: \n{err}"),
            MachineCodeError(err) => write!(f, "Machine code error: \n {err}"),
            FileError(err) => write!(f, ""),

            _ => write!(f, "Other error"),
        }
    }    
}

impl fmt::Display for MachineCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MachineCodeError::*;
        match self {
            ParseError(err) => write!(f, "Machine code parse error: \n{err}"),
            RawInstructionError => write!(f, ""),
            _ => write!(f, ""),
        }
    }    
}