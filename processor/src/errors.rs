



pub enum InputError {
    NoInputFileError,
    NoInteruptionScheduleError,
    InteruptScheduleParseError(serde_json::Error),
    MachineCodeError(MachineCodeError),
    FileError(std::io::Error)
}

pub enum MachineCodeError {
    ParseError(serde_json::Error),
    RawInstructionError(),
}