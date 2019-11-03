use ndarray::Array2;
pub type TypeIndex = u8;

#[derive(Debug)]
pub struct Types {
    pub names: Box<[String]>,
    pub chart: Array2<f32>
}

#[derive(Debug,Copy,Clone)]
pub enum MonsterType {
    Single(TypeIndex),
    Double(TypeIndex,TypeIndex),
}

impl Types {
    pub fn get(&self, name: &str) -> TypeIndex {
        self.names.iter().position(|x| x == name).unwrap() as TypeIndex
    }
    pub fn get_type_index_name(&self,type_id: TypeIndex) -> &str {
        &self.names[type_id as usize]
    }
    pub fn get_monster_type_name(&self,monster_type: MonsterType) -> &str {
        match monster_type {
            MonsterType::Single(type_id) => self.get_type_index_name(type_id),
            MonsterType::Double(_,_) => unimplemented!(),
        }
    }
    pub fn get_effectiveness(&self,attacker: TypeIndex, defender: MonsterType) -> f32 {
        match defender {
            MonsterType::Single(defender) => self.chart[[attacker as usize, defender as usize]],
            MonsterType::Double(main,sec) => {
                self.chart[[attacker as usize, main as usize]] *
                self.chart[[attacker as usize, sec as usize]]
            }
        }
    }

    pub fn new(names: Box<[String]>, chart: Vec<f32>) -> Types {
        let n = names.len();
        Types {
            names,
            chart: Array2::from_shape_vec((n,n),chart).unwrap()
        }
    }
}
