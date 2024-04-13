use core::fmt;

pub const COMMENT: &str = ";";
pub const INSTRUCTION_ARGUMENTS_SEPARATOR: &str = " ";
pub const STRING_QUOTE: &str = "\"";
pub const ARGUMENTS_SEPARATOR: &str = ",";
pub const SECTION: &str = "section";
pub const LABEL: &str = ":";


pub enum Section {
    Data,
    Code,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Section::Data => write!(f, ".data"),
            Section::Code => write!(f, ".code"),
        }
    }
}
