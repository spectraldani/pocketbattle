use pocketbattle::*;

fn read_crystal() -> System {
    System::from_file("./tests/crystal.toml").unwrap()
}

#[test]
fn create_chikorita() {
    let system = read_crystal();
    let chikorita_species = species::Species {
        name: "Chikorita".to_string(),
        typing: types::MonsterType::Single(7),
        stats: stats::Stats {hp:45,atk:49,def:65,spd:45,sat:49,sdf:65},
        happiness: 70,
        female_chance: 12.5,
        catch_rate: 45,
        hatch_cycles: 20,
        experience_growth: species::ExperienceGrowth::MediumSlow,
        egg_group: species::SpeciesEggGroup::Double(0,6),
    };

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
    assert_eq!(bulbasaur.current_stats, expected_stats);
}
