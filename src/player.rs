use crate::types::Gender;

pub struct Player {
    name: String,
    gender: Gender,
}

impl Player {
    pub fn new(name: String, gender: Gender) -> Self {
        Self {
            name: name,
            gender: gender
        }
    }
}