use std::fs;
pub mod types;
pub mod stats;
pub mod species;

#[derive(Debug)]
pub struct System {
    pub types: types::Types,
    pub species: Box<[species::Species]>
}

impl System {
    pub fn from_file(path: &str) -> Option<System> {
        let file_content = fs::read_to_string(path).expect("Expected file");
        let file_content = file_content.parse::<toml::Value>().expect("TOML parse failure");
        Some(System{
            types: types::Types::from_toml(file_content["types"].to_owned()),
            species: Box::new([])
        })
    }
}
