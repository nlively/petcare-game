use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use raylib::color::Color;
use crate::dog::Dog;
use crate::player::Player;
use std::time::{Instant, Duration};

const START_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
const GAME_TIME_PASSING_SPEED: f64 = 10.0; // one day in game time per 10 minutes of real world time

pub enum GameState {
    Initializing,
    Splash(SplashData),
    CollectingInfo,
    MainMenu,
    Playing
}

struct SplashData {
    start: Instant,
    duration: Duration
}

impl SplashData {
    fn new() -> Self { Self { start: Instant::now(), duration: Duration::from_secs(5) } }
    fn done(&self) -> bool { self.start.elapsed() >= self.duration }
}

pub struct Game {
    ticks_per_sec: i32,
    state: GameState,
    pub ticks: i32,
    pub dog: Option<Dog>,
    pub player: Option<Player>,
}

impl Game {
    pub fn new(ticks_per_sec: i32) -> Self {
        Self {
            ticks_per_sec: ticks_per_sec,
            ticks: 0,
            state: GameState::Initializing,
            dog: None,
            player: None
        }
    }

    pub fn show_splash(&mut self) {
        self.state = GameState::Splash(SplashData::new());
    }

    pub fn show_main_menu(&mut self) {
        self.state = GameState::MainMenu;
    }

    pub fn set_state(&mut self,state: GameState) {
        self.state = state;
    }

    pub fn date_in_game(&self) -> chrono::NaiveDate {
        // In the game universe, time moves at the rate of 1 day per 3 minutes
        let minutes_elapsed = f64::from(self.ticks / 3600);
        let days_elapsed  = (minutes_elapsed / GAME_TIME_PASSING_SPEED) as i64;

        let delta = chrono::TimeDelta::days(days_elapsed);

        START_DATE + delta
    }

    pub fn update(&mut self) {
        self.ticks += 1;

        match &self.state {
            GameState::Initializing => {
            },
            GameState::Splash(s) => {
                if s.done() {
                    self.show_main_menu();
                }
            },
            GameState::CollectingInfo => {
            },
            GameState::MainMenu => {
            },
            GameState::Playing => {
            },
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text("All my doggies", 12, 12, 20, Color::PINK);

        let status: &str;

        match self.state {
            GameState::Initializing => {
                status = "initalizing...";
            },
            GameState::Splash(_) => {
                status = "splash";
            },
            GameState::CollectingInfo => {
                status = "collecitng info";
            },
            GameState::MainMenu => {
                status = "main menu";
            },
            GameState::Playing => {
                status = "playing";
            },
        }

        d.draw_text(status, 12, 100, 12, Color::GREEN);
    }
}