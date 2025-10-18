// use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use std::time::{Instant, Duration};
use std::thread;

mod animation;
mod game;
mod dog;
mod types;
mod player;

const TICKS_PER_SEC: i32 = 60;


fn main() {
    let scottie = dog::Dog::new("Scottie".to_string(), dog::DogBreed::Cockapoo, types::Gender::Boy, chrono::NaiveDate::from_ymd_opt(2023, 11, 14).unwrap());
    let player = player::Player::new("noah".to_string(), types::Gender::Boy);

    // Setup game window
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("All My Doggies")
        .build();

    // Setup game data struct
    let mut game = game::Game::new(TICKS_PER_SEC, &mut rl, &thread);

    // Setup game timer
    let step = Duration::from_secs_f64(1.0 / TICKS_PER_SEC as f64);
    let mut next_tick = Instant::now();
    let mut accumulator = Duration::ZERO;
    let mut prev = Instant::now();

    game.show_splash();

    // these are just temp
    game.set_dog(scottie);
    game.set_player(player);

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
            game.update(&rl);
            accumulator -= step;
        }

        if game.is_quit() {
            break;
        }

        let mut d = rl.begin_drawing(&thread); // or however you begin your frame
        game.draw(&mut d);

        // pace to next tick boundary (optional but nice)
        next_tick += step;
        let now2 = Instant::now();
        if next_tick > now2 {
            // sleep most of the remaining time
            let remain = next_tick - now2;
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
