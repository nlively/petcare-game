use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use std::thread;
use std::time::{Instant, Duration};

mod game;
mod dog;
mod types;


const TICKS_PER_SEC: i32 = 60;


fn main() {


    let &mut scottie = dog::Dog {
        name: "Scottie",
        breed: dog::Cockapoo,
        gender: types::Boy,
        date_of_birth: chrono::NaiveDate::from_ymd_opt(2023, 11, 14),
        food_level: 100.0,
        water_level: 100.0,
        bladder_comfort: 100.0,
        digestion_comfort: 100.0,
        social_battery: 100.0,
        energy_level: 100.0,
        health_level: 100.0,
    };

    let &mut player = player::Player {
        name: "noah",
        gender: types::Boy,
    };

    // Setup game data struct
    let mut game = game::Game::new(TICKS_PER_SEC, scottie, player);

    // let kibble = Food{ name: "Kibble", nutritional_value: 3};
    // let rainbow_bone = Food { name: "Rainbow Bone", nutritional_value: 1};
    // let beef_scraps = Food { name: "Beef Scraps", nutritional_value: 5 };

    // game.dog.feed(kibble);

    // Setup game window
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("All My Doggies")
        .build();

    // Setup game timer
    let step = Duration::from_secs_f64(1.0 / TICKS_PER_SEC);
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
            game.update();
            accumulator -= step;
        }

        // render (optionally add interpolation alter)
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);

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
