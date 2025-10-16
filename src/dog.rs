
use crate::types::{Food, Gender, Percent, DrainRate};
use std::time::{Instant, Duration};

pub enum DogBreed {
    Pitbull,
    GermanShepherd,
    Corgie,
    AustralianShepherd,
    BorderCollie,
    BloodHound,
    CavalierKingCharlesSpaniel,
    Havanese,
    Husky,
    Mutt,
    Poodle,
    Labradoodle,
    Goldendoodle,
    Shepadoodle,
    Cockapoo,
    Schnauser,
    ScottishTerrier,
    Labrador,
    GoldenRetriever,
    SaintBernard,
    Greyhound,
    GreatDane,
    Mastiff,
    Dalmatian,
}

pub struct Dog {
    pub name: String,
    pub breed: DogBreed,
    pub gender: Gender,
    pub date_of_birth: chrono::NaiveDate,

    pub food_level: Percent,
    pub water_level: Percent,
    pub bladder_comfort: Percent,
    pub digestion_comfort: Percent,
    pub social_battery: Percent,
    pub energy_level: Percent,
    pub health_level: Percent,

    last_drain_applied: Instant,
    pub food_drain_rate: DrainRate,
    pub water_drain_rate: DrainRate,
}

const ONE_HOUR: Duration = Duration::from_secs(3600);

impl Dog {
    pub fn new(name: String, breed: DogBreed, gender: Gender, date_of_birth: chrono::NaiveDate) -> Self {
        Self {
            name: name,
            breed: breed,
            gender: gender,
            date_of_birth: date_of_birth,
            food_level: Percent::new(50.0),
            water_level: Percent::new(50.0),
            bladder_comfort: Percent::new(50.0),
            digestion_comfort: Percent::new(50.0),
            social_battery: Percent::new(50.0),
            energy_level: Percent::new(50.0),
            health_level: Percent::new(100.0),

            food_drain_rate: DrainRate::new(Percent::new(10.0), ONE_HOUR), // 10% food consumption per hour
            water_drain_rate: DrainRate::new(Percent::new(20.0), ONE_HOUR), // 20% water consumption per hour

            last_drain_applied: Instant::now(),
        }
    }

    pub fn feed(&mut self, food: &Food) {
        self.food_level.increase(food.nutritional_value.clone());
    }

    pub fn apply_drains(&mut self) {
        // don't do this more than once per second
        let elapsed = self.last_drain_applied.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            // drain food
            let base_food_drain = calculate_base_drain(&self.food_drain_rate, elapsed);
            self.food_level.decrease(base_food_drain);

            // drain water
            let base_water_drain = calculate_base_drain(&self.water_drain_rate, elapsed);
            self.water_level.decrease(base_water_drain);

            // drain bladder comfort 
            // drain digestion comfort
            // drain social battery
            // drain energy level
            // drain health level

            // update last drain time
            self.last_drain_applied = Instant::now();
        }
    }
}


fn calculate_base_drain(drain_rate: &DrainRate, elapsed: f64) -> Percent {
    let ratio = elapsed / drain_rate.duration.as_secs_f64();
    Percent::new(ratio * drain_rate.percent.as_f64())
}
