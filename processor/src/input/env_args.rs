const SOURCE_CODE_FILE_INDEX: usize = 1;
const INT_SCHEDULE_FILE_INDEX: usize = 2;
const LOG_FILE_INDEX: usize = 3;

pub struct EnvArgs {
    pub source_code: Option<String>,
    pub int_schedule: Option<String>,
    pub logs: Option<String>,
}

impl std::convert::From<Vec<String>> for EnvArgs {
    fn from(vec: Vec<String>) -> EnvArgs {
        EnvArgs {
            source_code: vec.get(SOURCE_CODE_FILE_INDEX).map(|string| string.clone()),
            int_schedule: vec.get(INT_SCHEDULE_FILE_INDEX).map(|string| string.clone()),
            logs: vec.get(INT_SCHEDULE_FILE_INDEX).map(|string| string.clone()),
        }
    }
}

impl EnvArgs {
    pub fn get() -> EnvArgs {
        EnvArgs::from(std::env::args().collect::<Vec<String>>())
    }
}