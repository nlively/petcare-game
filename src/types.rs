use std::time::Duration;

pub struct DrainRate {
    // think of this as: drain "unit" resource every "duration"
    pub percent: Percent, // relative to 100%, not current value
    pub duration: Duration,
}

impl DrainRate {
    pub fn new(percent: Percent, duration: Duration) -> Self {
        Self {
            percent: percent,
            duration: duration,
        }
    }
}

pub struct Percent(f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        Self(value / 100.0)
    }

    pub fn as_f64(&self) -> f64 { self.0 }

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

impl Clone for Percent {
    fn clone(&self) -> Self {
        Percent(self.0)
    }
}


pub enum Gender {
    Girl,
    Boy,
}



pub struct Food {
    pub name: String,
    pub nutritional_value: Percent,
}

impl Food {
    pub fn new(name: String, nutritional_value: Percent) -> Self {
        Self {
            name: name,
            nutritional_value: nutritional_value,
        }
    } 
}