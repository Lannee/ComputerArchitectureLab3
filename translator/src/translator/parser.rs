use crate::{errors::TranslationError, input::SourceCode};

use crate::translator::format::*;

use serde_json;

pub fn parse(code: SourceCode) -> Result<serde_json::Value, TranslationError> {



    todo!()
}


fn get_token(line: String) -> String {
        line
            .split(COMMENT_SYMBOL)
            .next()
            .unwrap()
            .trim()
            .to_string()
}


pub fn get_section_content(section: Section, code: String) -> Option<String> {
    Some(code
            .split(&format!("{}", SECTION))
            .map(|x| x.trim())
            .filter(|&x| x.starts_with(&section.to_string()))
            .next()?
            .strip_prefix(&section.to_string())
            .unwrap()
            .trim()
            .to_string()
        )
}