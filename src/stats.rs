use std::ops::Index;

#[derive(Debug)]
pub enum Stat {
    Hp,
    Attack,
    Defense,
    Speed,
    SpecialAttack,
    SpecialDefense
}

#[derive(Debug)]
pub struct Stats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spd: u8,
    pub sat: u8,
    pub sdf: u8,
}

impl Index<Stat> for Stats {
    type Output = u8;
    fn index(&self, stat: Stat) -> &Self::Output {
        match stat {
            Stat::Hp => &self.hp,
            Stat::Attack => &self.atk,
            Stat::Defense => &self.def,
            Stat::Speed => &self.spd,
            Stat::SpecialAttack => &self.sat,
            Stat::SpecialDefense => &self.sdf,
        }
    }
}
