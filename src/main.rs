#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split::{engine::app::app::App, game::game::Game};

fn main() {
    let _ = App::<Game>::new()
        .run();
}