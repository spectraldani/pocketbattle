pub mod types;
pub mod stats;
pub mod attack;
pub mod species;
pub mod monster;

#[derive(Debug)]
pub struct System {
    pub types: types::Types,
    pub attacks: attack::Attacks,
    pub species: species::SpeciesList,
}
