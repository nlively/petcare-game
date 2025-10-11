
pub struct Percent(f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        Self(value / 100.0)
    }

    pub fn increase(&mut self, value: Percent) {
        if self.0 + value.0 > 100.0 {
            self.0 = 100.0
        } else {
            self.0 += value.0
        }
    }

    pub fn decrease(&mut self, value: Percent) {
        if self.0 - value.0 < 0.0 {
            self.0 = 0.0
        } else {
            self.0 -= value.0
        }
    }
}



pub enum Gender {
    Girl,
    Boy,
}


pub struct Player {
    name: String,
    gender: Gender,
}

pub struct Food {
    name: String,
    nutritional_value: f64,
}