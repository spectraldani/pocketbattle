use super::stats::{Stat, Stats, IndividualValues};
use super::species::{Species};

#[derive(Debug,PartialEq,Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug,PartialEq,Eq)]
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
pub struct Monster<'sys> {
    pub nickname: String,
    pub species: &'sys Species,
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

impl<'sys> Monster<'sys> {
    pub fn from_species(species: &'sys Species) -> Monster {
        let ivs = IndividualValues::zeroes(); // todo
        let gender = if species.female_chance.is_finite() {
            match (ivs.get(Stat::Attack) as f32) < species.female_chance*15. {
                true => Some(Gender::Female),
                false => Some(Gender::Male),
            }
        } else {
            None
        };
        Monster {
            nickname: species.name.clone(),
            species: species,
            gender: gender,
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
