use core::fmt;

pub const COMMENT_SYMBOL: &str = ";";
pub const SECTION: &str = "section";


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
