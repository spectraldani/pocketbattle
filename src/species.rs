use super::stats::{Stats, StatFormula};
use super::types::{MonsterType,TypeIndex};
use serde::Deserialize;
pub type Level = u8;
pub type EggGroupIndex = u8;
pub type SpeciesIndex = u8;
type ItemIndex = u8;
type MoveIndex = u8;

#[derive(Debug,Deserialize)]
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
    Level(Level,SpeciesIndex),
    // Item(ItemIndex, SpeciesIndex),
    // Trade(Option<ItemIndex>, SpeciesIndex),
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
    pub happiness: u8,
    // pub held_itens: Box<[ItemIndex]>,
    pub female_chance: f32,
    pub catch_rate: u8,
    pub hatch_cycles: u8,
    pub experience_growth: ExperienceGrowth,
    pub egg_group: SpeciesEggGroup,
    // pub level_up_moves: Box<[(Level,MoveIndex)]>,
    // pub machine_moves: Box<[MoveIndex]>
    // pub evolutions: Box<[EvolutionTransition]>,
}

#[derive(Debug,Deserialize)]
struct SerializedEvolutionTransition {
    how: String,
    level: Level,
    species: SpeciesIndex
}

#[derive(Debug,Deserialize)]
struct SerializedLevelUpMoves {
    level: Level,
    r#move: MoveIndex,
}

#[derive(Debug,Deserialize)]
struct SerializedSpecies {
    name: String,
    typing: Box<[TypeIndex]>,
    stats: Stats,
    female_chance: f32,
    catch_rate: u8,
    hatch_cycles: u8,
    happiness: u8,
    experience_growth: ExperienceGrowth,
    held_itens: Box<[ItemIndex]>,
    egg_groups: Box<[EggGroupIndex]>,
    level_up_moves: Box<[SerializedLevelUpMoves]>,
    evolutions: Box<[SerializedEvolutionTransition]>,
}

impl Species {
    pub fn from_toml(raw_species: toml::Value) -> Species {
        let raw_species = raw_species.try_into::<SerializedSpecies>().unwrap();
        let typing = match *raw_species.typing {
            [a] => MonsterType::Single(a),
            [a,b] => MonsterType::Double(a,b),
            _ => unreachable!()
        };
        let egg_group = match *raw_species.egg_groups {
            [a] => SpeciesEggGroup::Single(a),
            [a,b] => SpeciesEggGroup::Double(a,b),
            _ => unreachable!()
        };
        Species {
            name: raw_species.name,
            typing: typing,
            stats: raw_species.stats,
            happiness: raw_species.happiness,
            female_chance: raw_species.female_chance,
            catch_rate: raw_species.catch_rate,
            hatch_cycles: raw_species.hatch_cycles,
            experience_growth: raw_species.experience_growth,
            egg_group: egg_group,
        }
    }
}
