use std::fs;
pub mod types;
pub mod stats;
pub mod species;
pub mod monster;

#[derive(Debug)]
pub struct System {
    pub types: types::Types,
    pub species: Box<[species::Species]>
}

impl System {
    pub fn from_file(path: &str) -> Option<System> {
        let file_content = fs::read_to_string(path).expect("Expected file");
        let file_content = file_content.parse::<toml::Value>().expect("TOML parse failure");

        let types = types::Types::from_toml(file_content["types"].to_owned());
        let species = file_content["species"].as_array().expect("Should be array").iter().map(
            |x| species::Species::from_toml(x.to_owned())
        ).collect();

        Some(System{types,species})
    }
}
