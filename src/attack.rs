use super::types::{TypeIndex};
pub type AttackIndex = u8;

#[derive(Debug)]
pub struct Attack {
    pub name: String,
    pub typing: TypeIndex,
    pub power: u8,
    pub pp: u8,
    pub effect: AttackEffect
}

#[derive(Debug)]
pub enum AttackEffect {
    NormalHit,
}

#[derive(Debug)]
pub struct Attacks(pub Box<[Attack]>);
impl Attacks {
    pub fn get(&self, name: &str) -> &Attack {
        self.0.iter().find(|&x| x.name == name).unwrap()
    }
}
