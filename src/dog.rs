
use crate::types::{Food, Gender, Percent, DrainRate};
use crate::animation::{Facing, Pose, Emotion, AnimationDescriptor, AnimationPlayer};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Rectangle, Vector2};


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

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct AnimationKey {
    pub pose: Pose,
    pub emotion: Emotion,
    pub facing: Facing,
}

pub struct Dog {
    pub name: String,
    pub breed: DogBreed,
    pub gender: Gender,
    pub date_of_birth: chrono::NaiveDate,

    pub position: Vector2,
    pub velocity: Vector2,

    pub facing: Facing,
    pub pose: Pose,
    pub emotion: Emotion,

    // shared animation descriptors live in a map filled at load time
    pub animations: HashMap<AnimationKey, AnimationDescriptor>,

    // the per-dog player that uses the current animation descripttor
    pub sprite_player: Option<AnimationPlayer>,

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
        let animations: HashMap<AnimationKey, AnimationDescriptor>;

        // todo: fix this
        animations = HashMap::new();

        Self {
            name: name,
            breed: breed,
            gender: gender,
            date_of_birth: date_of_birth,

            animations: animations,
            sprite_player: None,

            emotion: Emotion::Neutral,
            pose: Pose::Standing,
            facing: Facing::Right,

            // position and evelocity
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },

            // init levels
            food_level: Percent::new(50.0),
            water_level: Percent::new(50.0),
            bladder_comfort: Percent::new(50.0),
            digestion_comfort: Percent::new(50.0),
            social_battery: Percent::new(50.0),
            energy_level: Percent::new(50.0),
            health_level: Percent::new(100.0),

            // init drain rates
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

    // call this once after loading descriptors
    pub fn init_sprite_player(&mut self) {
        let key = AnimationKey { pose: self.pose, emotion: self.emotion, facing: self.facing };
        if let Some(descriptor) = self.animations.get(&key) {
            // clone desc into palyer (desc contains the texture2D so adjust as needed)
            self.sprite_player = Some(AnimationPlayer::new(descriptor.clone()));
        }
    }

    // call when visual state changes to switch animations
    pub fn set_visual_state(&mut self, pose: Pose, emotion: Emotion, facing: Facing) {
        let key = AnimationKey { pose, emotion, facing };
        if let Some(descriptor) = self.animations.get(&key) {
            match &mut self.sprite_player {
                Some(player) => {
                    // if different descriptor, replace/reset
                    // here we simply compare pointer/texture or replace unconditionally
                    player.descriptor = descriptor.clone();
                    player.reset();
                },
                None => {
                    self.sprite_player = Some(AnimationPlayer::new(descriptor.clone()));
                }
            }
        }
        self.pose = pose;
        self.emotion = emotion;
        self.facing = facing;
    }

    // update movement + sprite each frame
    // dt in seconds
    pub fn update(&mut self, dt: f32) {
        // basic movement example
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        // select pose by velocity
        let moving = (self.velocity.x.abs() + self.velocity.y.abs()) > 0.01;
        let pose = if moving { Pose::Walking } else { self.pose }; // retain existing pose unless movement detected

        // facing from velocity.x if moving
        let facing = if self.velocity.x > 0.0 { Facing::Right }
                     else if self.velocity.x < 0.0 { Facing::Left }
                     // todo: i might have these backwards
                     else if self.velocity.y > 0.0 { Facing::Back }
                     else if self.velocity.y < 0.0 { Facing::Front }
                     else { self.facing };

        // update animation if needed
        if pose != self.pose || facing != self.facing {
            self.set_visual_state(pose, self.emotion, facing);
        }

        if let Some(player) = &mut self.sprite_player {
            player.update(dt);
        }
    }

    // draw the dog using the player's current frame
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if let Some(player) = &self.sprite_player {
            let src = player.current_frame_rect();
            // destination rect (example: keep source size, place at position)
            let dest_rect = Rectangle::new(self.position.x, self.position.y, src.width, src.height);
            // origin for rotation/scale - top-left here ;adjust to center if desired
            let origin = Vector2::new(0.0, 0.0);

            d.draw_texture_pro(
                &player.texture(),
                src,
                dest_rect,
                origin,
                0.0,
                raylib::color::Color::WHITE
            );
        }
    }
}


fn calculate_base_drain(drain_rate: &DrainRate, elapsed: f64) -> Percent {
    let ratio = elapsed / drain_rate.duration.as_secs_f64();
    Percent::new(ratio * drain_rate.percent.as_f64())
}
