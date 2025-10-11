#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split;

use crate::{platform::{app::App, introduction}, scenes::basic_particles::BasicParticles};

pub mod platform;
pub mod particles;
pub mod math;
pub mod scenes;
pub mod constraints;
pub mod level;
pub mod entity;

fn main() {
    //platform::app_inner::run().unwrap();

    // Follow Bevy's API
    let _ = App::new()
        .add_plugin(Box::new(BasicParticles::new()))
        .run();

    //sort::run().unwrap();
    //introduction::run().unwrap();
}