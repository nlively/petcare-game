use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use raylib::color::Color;
use crate::dog::Dog;
use crate::player::Player;

const START_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
const GAME_TIME_PASSING_SPEED: f64 = 10.0; // one day in game time per 10 minutes of real world time


pub struct Game {
    ticks_per_sec: i32,
    pub ticks: i32,
    pub dog: Dog,
    pub player: Player,
}

impl Game {
    pub fn new(ticks_per_sec: i32, dog: crate::dog::Dog, player: Player) -> Self {
        Self {
            ticks_per_sec: ticks_per_sec,
            ticks: 0,
            dog: dog,
            player: player
        }
    }

    pub fn date(&self) -> chrono::NaiveDate {
        // In the game universe, time moves at the rate of 1 day per 3 minutes
        let minutes_elapsed = f64::from(self.ticks / 3600);
        let days_elapsed  = (minutes_elapsed / GAME_TIME_PASSING_SPEED) as i64;

        let delta = chrono::TimeDelta::days(days_elapsed);

        START_DATE + delta
    }

    pub fn update(&mut self) {
        self.ticks += 1;

    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("All my doggies {}", self.ticks as f64 / self.ticks_per_sec as f64), 12, 12, 20, Color::PINK);
    }
}