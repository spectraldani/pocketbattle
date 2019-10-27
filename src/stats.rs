use std::ops::Index;
use serde::Deserialize;

#[derive(Debug)]
pub enum Stat {
    Hp,
    Attack,
    Defense,
    Speed,
    SpecialAttack,
    SpecialDefense
}

#[derive(Debug,PartialEq,Copy,Clone,Deserialize)]
pub struct Stats {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub sat: u16,
    pub sdf: u16,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct IndividualValues {
    pub atk_def: u8,
    pub spd_spe: u8,
}

#[derive(Debug)]
pub enum StatFormula {
    LessThan(Stat,Stat),
    GreaterThan(Stat,Stat),
    Equals(Stat,Stat),
}


impl Stats {
    pub fn zeroes() -> Stats {
        Stats {hp:0,atk:0,def:0,spd:0,sat:0,sdf:0}
    }

    pub fn from_base(base: Stats, ev: Stats, iv: IndividualValues, level: u8) -> Stats {
        let level = level as u16;
        Stats {
            hp: (level*((base.hp + iv.get(Stat::Hp))*2 + isqrt(ev.hp)/4))/100 + level + 10,
            atk: (level*((base.atk + iv.get(Stat::Attack))*2 + isqrt(ev.atk)/4))/100 + 5,
            def: (level*((base.def + iv.get(Stat::Defense))*2 + isqrt(ev.def)/4))/100 + 5,
            spd: (level*((base.spd + iv.get(Stat::Speed))*2 + isqrt(ev.spd)/4))/100 + 5,
            sat: (level*((base.sat + iv.get(Stat::SpecialAttack))*2 + isqrt(ev.sat)/4))/100 + 5,
            sdf: (level*((base.sdf + iv.get(Stat::SpecialDefense))*2 + isqrt(ev.sdf)/4))/100 + 5,
        }
    }
}

impl IndividualValues {
    pub fn zeroes() -> IndividualValues {
        IndividualValues {atk_def: 0, spd_spe: 0}
    }

    pub fn get(&self, stat: Stat) -> u16 {
        (match stat {
            Stat::Hp => (
                self.atk_def & 0x10 << 3 +
                self.atk_def & 0x01 << 2+
                self.spd_spe & 0x10 << 1 +
                self.spd_spe & 0x01
            ),
            Stat::Attack =>  (&self.atk_def & 0xf0) >> 4,
            Stat::Defense => (&self.atk_def & 0x0f),
            Stat::Speed =>          (&self.spd_spe & 0xf0) >> 4,
            Stat::SpecialAttack =>  (&self.spd_spe & 0x0f),
            Stat::SpecialDefense => (&self.spd_spe & 0x0f),
        }) as u16
    }
}

impl Index<Stat> for Stats {
    type Output = u16;
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

fn isqrt(x: u16) -> u16 {
    (x as f32).sqrt() as u16
}
