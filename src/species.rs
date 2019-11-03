use std::ops::Index;

use super::stats::{Stats, StatFormula};
use super::types::{MonsterType};
use super::attack::{Attack};
pub type Level = u8;
pub type EggGroupIndex = u8;
pub type SpeciesIndex = u8;

#[derive(Debug)]
pub enum ExperienceGrowth {
    Slow,
    MediumSlow,
    MediumFast,
    Fast,
    Erratic
}

#[derive(Debug)]
pub enum HappinessEvolveTime {
    Anytime,
    Morning,
    Night,
}

#[derive(Debug)]
pub enum EvolutionTransition {
    Level{ level: Level, species: SpeciesIndex},
    // Item(ItemIndex, SpeciesIndex),
    // Trade(Option<ItemIndex>, SpeciesIndex),
    Happiness{ time: HappinessEvolveTime, species: SpeciesIndex },
    Stat{ level: Level, formula: StatFormula, species: SpeciesIndex},
}

#[derive(Debug)]
pub enum SpeciesEggGroup {
    None,
    Single(EggGroupIndex),
    Double(EggGroupIndex, EggGroupIndex),
}

#[derive(Debug)]
pub struct Species {
    pub name: String,
    pub typing: MonsterType,
    pub stats: Stats,
    pub happiness: u8,
    // pub held_itens: Box<[ItemIndex]>,
    pub female_chance: f32,
    pub catch_rate: u8,
    pub hatch_cycles: u8,
    pub experience_growth: ExperienceGrowth,
    pub egg_group: SpeciesEggGroup,
    // pub level_up_attacks: Box<[LevelUpAttacks]>,
    // pub machine_attacks: Box<[AttackIndex]>
    pub evolutions: Box<[EvolutionTransition]>,
}

#[derive(Debug)]
struct LevelUpAttacks<'sys> {
    level: Level,
    attack: &'sys Attack,
}

#[derive(Debug)]
pub struct SpeciesList(pub Box<[Species]>);
impl SpeciesList {
    pub fn get(&self, name: &str) -> &Species {
        self.0.iter().find(|&x| x.name == name).unwrap()
    }
}
impl Index<usize> for SpeciesList {
    type Output = Species;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}
