const SOURCE_FILE_INDEX: usize = 1;
const OUT_FILE_INDEX: usize = 2;

pub struct EnvArgs {
    pub source_file: Option<String>,
    pub out_file: Option<String>
}

impl std::convert::From<Vec<String>> for EnvArgs {
    fn from(vec: Vec<String>) -> EnvArgs {
        EnvArgs {
            source_file: vec.get(SOURCE_FILE_INDEX).map(|string| string.clone()),
            out_file: vec.get(OUT_FILE_INDEX).map(|string| string.clone()),
        }
    }
}

impl EnvArgs {
    pub fn get() -> EnvArgs {
        EnvArgs::from(std::env::args().collect::<Vec<String>>())
    }
}