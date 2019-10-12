use pocketbattle::*;

fn read_crystal() -> System {
    System::from_file("./tests/crystal.toml").unwrap()
}

#[test]
fn create_chikorita() {
    let system = read_crystal();
    let chikorita = species::Species {
        name: "Chikorita".to_string(),
        typing: types::MonsterType::Single(7),
        stats: stats::Stats {hp:45,atk:49,def:65,spd:45,sat:49,sdf:65},
        female_chance: 12.5,
        capture_rate: 45,
        happiness: 70,
        experience_growth: species::ExperienceGrowth::MediumSlow,
        // held_itens: Box<[ItemIndex]>,
        egg_group: species::SpeciesEggGroup::Double(0,6),
        // level_up_moves: Box<[(Level,MoveIndex)]>,
        // evolutions: Box<[EvolutionTransition]>,
    };

    println!("{:?}", chikorita);
    assert!(system.types.get_monster_type_name(chikorita.typing) == "Grass");
}
