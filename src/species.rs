use super::stats::{Stat, Stats};
use super::types::{MonsterType};
pub type Level = u8;
pub type MoveIndex = u8;
pub type ItemIndex = u8;
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
pub enum StatFormula {
    LessThan(Stat,Stat),
    GreaterThan(Stat,Stat),
    Equals(Stat,Stat),
}

#[derive(Debug)]
pub enum EvolutionTransition {
    Level(Level,SpeciesIndex),
    Item(ItemIndex, SpeciesIndex),
    Trade(Option<ItemIndex>, SpeciesIndex),
    Happiness(HappinessEvolveTime, SpeciesIndex),
    Stat(Level, StatFormula, SpeciesIndex),
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
    pub female_chance: f32,
    pub capture_rate: u8,
    pub happiness: u8,
    pub experience_growth: ExperienceGrowth,
    // pub held_itens: Box<[ItemIndex]>,
    pub egg_group: SpeciesEggGroup,
    // pub level_up_moves: Box<[(Level,MoveIndex)]>,
    // pub evolutions: Box<[EvolutionTransition]>,
}
