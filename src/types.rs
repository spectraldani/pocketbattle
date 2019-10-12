use ndarray::Array2;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct SerializedTypes {
    ammount: u8,
    names: Box<[String]>,
    chart: Vec<f32>
}

#[derive(Debug)]
pub struct Types {
    names: Box<[String]>,
    chart: Array2<f32>
}
pub type TypeIndex = u8;

#[derive(Debug)]
pub enum MonsterType {
    Single(TypeIndex),
    Double(TypeIndex,TypeIndex),
}

impl Types {
    pub fn get_type_index_name(&self,type_id: TypeIndex) -> &str {
        &self.names[type_id as usize]
    }
    pub fn get_monster_type_name(&self,monster_type: MonsterType) -> &str {
        match monster_type {
            MonsterType::Single(type_id) => self.get_type_index_name(type_id),
            MonsterType::Double(_,_) => unimplemented!(),
        }
    }
    pub fn get_attack_effectiveness(&self,attacker: TypeIndex, defender: MonsterType) -> f32 {
        match defender {
            MonsterType::Single(defender) => self.chart[[attacker as usize, defender as usize]],
            MonsterType::Double(main,sec) => {
                self.chart[[attacker as usize, main as usize]] *
                self.chart[[attacker as usize, sec as usize]]
            }
        }
    }
    pub fn from_toml(raw_types: toml::Value) -> Types {
        let raw_types = raw_types.try_into::<SerializedTypes>().unwrap();
        let dim = (raw_types.ammount as usize, raw_types.ammount as usize);
        Types {
            names: raw_types.names,
            chart: Array2::from_shape_vec(dim,raw_types.chart).expect(
                "Typechart with wrong dimensions"
            ),
        }
    }
}
