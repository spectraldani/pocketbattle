use std::fs;
pub mod types;

#[derive(Debug)]
pub struct System {
    types: types::Types
}

impl System {
    pub fn from_file(path: &str) -> Option<System> {
        let file_content = fs::read_to_string(path).expect("Expected file");
        let file_content = file_content.parse::<toml::Value>().expect("TOML parse failure");
        Some(System{
            types: types::Types::from_toml(file_content["types"].to_owned())
        })
    }
}
