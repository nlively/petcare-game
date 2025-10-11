
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
}

impl Dog {
    pub fn new(g: &Game, name: String, breed: DogBreed, gender: Gender) -> Self {
        Self {
            name: name,
            breed: breed,
            gender: gender,
            date_of_birth: g.date(),
            food_level: Percent::new(50.0),
            water_level: Percent::new(50.0),
            bladder_comfort: Percent::new(50.0),
            digestion_comfort: Percent::new(50.0),
            social_battery: Percent::new(50.0),
            energy_level: Percent::new(50.0),
            health_level: Percent::new(100.0),
        }
    }

    pub fn feed(&mut self, food: &Food) {
        // max nutritional value is 10.
        // the percentage increases by the food's nutritional value / 10.
        let pct = Percent::new(food.nutritional_value / 10.0);
        self.food_level.increase(pct);
    }
}