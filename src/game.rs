use raylib::prelude::{RaylibDraw, RaylibDrawHandle, RaylibHandle, KeyboardKey};
use raylib::color::Color;
use crate::dog::Dog;
use crate::player::Player;
use crate::types::{Food,Percent};
use std::time::{Instant, Duration};

const START_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
const GAME_TIME_PASSING_SPEED: f64 = 10.0; // one day in game time per 10 minutes of real world time

pub enum GameState {
    Initializing,
    Splash(SplashData),
    CollectingInfo,
    MainMenu,
    Playing,
    Paused,
    Quit,
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

    pub fn is_quit(&self) -> bool {
        matches!(self.state, GameState::Quit)
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

    pub fn set_dog(&mut self, dog: Dog) {
        self.dog = Some(dog);
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    pub fn date_in_game(&self) -> chrono::NaiveDate {
        // In the game universe, time moves at the rate of 1 day per 3 minutes
        let minutes_elapsed = f64::from(self.ticks / 3600);
        let days_elapsed  = (minutes_elapsed / GAME_TIME_PASSING_SPEED) as i64;

        let delta = chrono::TimeDelta::days(days_elapsed);

        START_DATE + delta
    }

    fn update_main_menu(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            // TODO: actually, we should be making sure the player and dog are initialized before starting the game
            self.set_state(GameState::Playing);
        }
    }

    fn update_playing(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            // pause on spacebar
            self.set_state(GameState::Paused);
        } else if rl.is_key_pressed(KeyboardKey::KEY_F) {
            match &mut self.dog {
                Some(dog) =>  {
                    let food = Food::new("kibble".to_string(), Percent::new(15.0));
                    dog.feed(&food);
                },
                None => {}
            }
        } else {
            match &mut self.dog {
                Some(dog) => dog.apply_drains(),
                None => {}
            }
        }
    }

    fn update_paused(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            // unpause on spacebar
            self.set_state(GameState::Playing);
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.ticks += 1;

        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
           self.set_state(GameState::Quit)
        }

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
                self.update_main_menu(rl);
            },
            GameState::Playing => {
                self.update_playing(rl);
            },
            GameState::Paused => {
                self.update_paused(rl);
            },
            GameState::Quit => {
                // do nothing. main.rs will take it from here.
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
                status = "collecting info";
            },
            GameState::MainMenu => {
                status = "main menu";
            },
            GameState::Playing => {
                status = "playing";
                match &self.dog {
                    Some(dog) => {
                        let text = format!("food level {}", dog.food_level.as_f64());
                        d.draw_text(&text, 12, 150, 12, Color::YELLOW);
                    },
                    None => {}
                }
                
            },
            GameState::Paused => {
                status = "paused";
            },
            GameState::Quit => {
                status = "quitting";
            }
        }

        d.draw_text(status, 12, 100, 12, Color::GREEN);
    }
}