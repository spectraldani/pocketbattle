#![feature(box_syntax)]

use pocketbattle::*;

macro_rules! string_box {
    ($($x:expr),*) => (box [$($x.to_string()),*]);
}

fn read_crystal() -> System {
    let types = types::Types::new(
        string_box![
            "Normal","Fighting","Flying","Poison","???","Fire","Water","Grass","Electric",
            "Psychic","Ice","Ground","Rock","Bug","Ghost","Steel","Dragon","Dark"
        ],
        vec![
          1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.0, 0.5, 1.0, 1.0,
          2.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 0.5, 0.0, 2.0, 1.0, 2.0,
          1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0,
          1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 0.5, 0.0, 1.0, 1.0,
          1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
          1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 2.0, 1.0, 2.0, 0.5, 1.0,
          1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 0.5, 1.0,
          1.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 0.5, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0, 0.5, 0.5, 1.0,
          1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0,
          1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.0,
          1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 2.0, 1.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0,
          1.0, 1.0, 0.0, 2.0, 1.0, 2.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 2.0, 1.0, 1.0,
          1.0, 0.5, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0,
          1.0, 0.5, 0.5, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 2.0,
          0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5,
          1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0,
          1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0,
          1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5,
        ]
    );

    let attacks = attack::Attacks(box [
        attack::Attack {
            name: "Tackle".into(), typing: types.get("Normal"),
            power: 35, pp: 35,
            effect: attack::AttackEffect::NormalHit
        }
    ]);

    let species = species::SpeciesList(box [
        species::Species {
            name: "Bulbasaur".into(),
            typing: types::MonsterType::Single(types.get("Grass")),
            stats: stats::Stats {hp:45,atk:49,def:49,spd:45,sat:65,sdf:65},
            happiness: 70,
            female_chance: 12.5,
            catch_rate: 45,
            hatch_cycles: 20,
            experience_growth: species::ExperienceGrowth::Slow,
            egg_group: species::SpeciesEggGroup::Double(0,6),
            evolutions: box [
                species::EvolutionTransition::Level{level: 16, species: 1}
            ]
        },
        species::Species {
            name: "Chikorita".into(),
            typing: types::MonsterType::Single(types.get("Grass")),
            stats: stats::Stats {hp:45,atk:49,def:65,spd:45,sat:49,sdf:65},
            happiness: 70,
            female_chance: 12.5,
            catch_rate: 45,
            hatch_cycles: 20,
            experience_growth: species::ExperienceGrowth::MediumSlow,
            egg_group: species::SpeciesEggGroup::Double(0,6),
            evolutions: box [
                species::EvolutionTransition::Level{level: 16, species: 152}
            ]
        }
    ]);

    System { types, attacks, species, }
}

#[test]
fn check_chikorita() {
    let system = read_crystal();
    let chikorita_species = system.species.get("Chikorita");
    println!("{:?}", chikorita_species);
    assert!(system.types.get_monster_type_name(chikorita_species.typing) == "Grass");
}

#[test]
fn create_bulbasaur_monster() {
    let system = read_crystal();
    let bulbasaur_species = &system.species[0];
    let bulbasaur = monster::Monster::from_species(&bulbasaur_species);

    let expected_stats = stats::Stats {
        hp: 19, atk: 9, def: 9, spd: 9, sat: 11, sdf: 11
    };
    assert_eq!(bulbasaur.nickname, "Bulbasaur");
    assert_eq!(bulbasaur.current_stats, expected_stats);
    assert_eq!(bulbasaur.gender, Some(monster::Gender::Female));
}
