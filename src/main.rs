#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use planck_time_trials::{engine::app::app::App, game::game::Game};

fn main() {
    let _ = App::<Game>::new()
        .run();
}