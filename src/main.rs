use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use raylib::color::Color;
use std::thread;
use std::time::{Instant, Duration};

const TicksPerSec: f64 = 60.0;
const GameTimePassingSpeed: f64 = 3.0; // one day in game time per 3 minutes of real world time
const StartDate: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

struct Percent(f64);

impl Percent {
    fn new(value: f64) -> Self {
        Self(value / 100.0)
    }
}

enum DogBreed {
    Pitbull,
    GermanShepherd,
    Corgie,
    AustralianShepherd,
    BorderCollie,
    BloodHound,
    CavalierKingCharlesSpaniel,
    Havanese,
    Husky,
    Wolf,
    Poodle,
    Labradoodle,
    Goldendoodle,
    Shepadoodle,
    Schnauser,
    ScottishTerrier,
    Labrador,
    GoldenRetriever,
    SaintBernard,
    Greyhound,
    GreatDane,
    Mastiff
}

struct Dog {
    name: String,
    breed: DogBreed,
    date_of_birth: chrono::NaiveDate,
    food_level: Percent,
    water_level: Percent,
    bladder_comfort: Percent,
    digestion_comfort: Percent,
    social_battery: Percent,
    energy_level: Percent,
}

impl Dog {
    fn new(g: &Game, name: String, breed: DogBreed) -> Self {
        Self {
            name: name,
            breed: breed,
            date_of_birth: g.date(),
            food_level: Percent::new(50.0),
            water_level: Percent::new(50.0),
            bladder_comfort: Percent::new(50.0),
            digestion_comfort: Percent::new(50.0),
            social_battery: Percent::new(50.0),
            energy_level: Percent::new(50.0),
        }
    }
    fn feed(&mut self) {

    }
}

struct Game {
    ticks: i32,
}

impl Game {
    fn date(&self) -> chrono::NaiveDate {
        // In the game universe, time moves at the rate of 1 day per 3 minutes
        let minutes_elapsed = f64::from(self.ticks / 3600);
        let days_elapsed  = (minutes_elapsed / GameTimePassingSpeed) as i64;

        let delta = chrono::TimeDelta::days(days_elapsed);

        StartDate + delta
    }

    fn update(&mut self) {
        self.ticks += 1;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("All my doggies {}", self.ticks as f64 / TicksPerSec), 12, 12, 20, Color::PINK);
    }
}


fn main() {
    // Setup game data struct
    let mut g = Game {
        ticks: 0,
    };

    // Setup game window
    let (mut rl, thread) = raylib::init()
    .size(800, 600)
    .title("All My Doggies")
    .build();

    // Setup game timer
    let step = Duration::from_secs_f64(1.0 / TicksPerSec);
    let mut next_tick = Instant::now();
    let mut accumulator = Duration::ZERO;
    let mut prev = Instant::now();

    while !rl.window_should_close() {

        let now = Instant::now();
        let frame_dt = now - prev;
        prev = now;

        // clamp huge pauses
        let max_frame = Duration::from_millis(250);
        let clamped = if frame_dt > max_frame { max_frame } else { frame_dt };
        accumulator += clamped;

        // fixed updates
        while accumulator >= step {
            g.update();
            accumulator -= step;
        }

        // render (optionally add interpolation alter)
        let mut d = rl.begin_drawing(&thread);
        g.draw(&mut d);

        // pace to next tick boundary (optional but nice)
        next_tick += step;
        let now2 = Instant::now();
        if next_tick > now2 {
            // sleep most of the remaining time
            let mut remain = next_tick - now2;
            if remain > Duration::from_micros(500) {
                thread::sleep(remain - Duration::from_micros(300)); // leave small buffer
            }

            // short spin to hint the boundary more precisely
            while Instant::now() < next_tick {
                std::hint::spin_loop();
            }
        } else {
            // we're behind. realtime catch-up happens via the while-accumulator loop
            next_tick = Instant::now();
        }
    }
}
