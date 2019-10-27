use super::stats::{Stats, IndividualValues};
use super::species::{Species};

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub enum StatusCondition {
    SLP,
    PSN,
    BRN,
    FRZ,
    PAR
}

#[derive(Debug)]
pub struct InfectionStatus {
    xy: u8
}
impl InfectionStatus {
    pub fn uninfected() -> InfectionStatus {
        InfectionStatus {
            xy: 0
        }
    }
    pub fn get_strain(&self) -> u8 {
        (&self.xy & 0b11110000) >> 4
    }
    pub fn get_remaining_days(&self) -> u8 {
        (&self.xy & 0b00001111)
    }
}

#[derive(Debug)]
pub struct Monster<'system_lifetime> {
    pub nickname: String,
    pub species: &'system_lifetime Species,
    pub gender: Option<Gender>,
    // pub original_trainer_data: TrainerData,
    // pub held_item: Option<ItemIndex>,
    // pub moves: [Option<(MoveIndex, u8)>; 4],
    pub effort_values: Stats,
    pub individual_values: IndividualValues,
    pub friendship: u8,
    pub infection_status: InfectionStatus,
    // pub caught_data: ???,
    pub level: u8,
    pub status_condition: Option<StatusCondition>,
    pub current_stats: Stats
}

impl<'system_lifetime> Monster<'system_lifetime> {
    pub fn from_species(species: &'system_lifetime Species) -> Monster {
        let ivs = IndividualValues::zeroes(); // todo
        Monster {
            nickname: species.name.clone(),
            species: species,
            gender: None, // todo
            effort_values: Stats::zeroes(),
            individual_values: ivs,
            friendship: 0,
            infection_status: InfectionStatus::uninfected(),
            level: 5,
            status_condition: None,
            current_stats: Stats::from_base(species.stats, Stats::zeroes(), ivs, 5)
        }
    }
}
