#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split::engine::app::app::App;
use merge_and_split::game::scenes::basic_particles::BasicParticles;

fn main() {
    // Follow Bevy's API
    // Follow Bevy's API
    let _ = App::<BasicParticles>::new()
        .run();
}