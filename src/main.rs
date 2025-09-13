#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split;

use crate::platform::{app::App, introduction};

pub mod platform;
pub mod particles;
pub mod math;


fn main() {
    //platform::app_inner::run().unwrap();

    // Follow Bevy's API
    let _ = App::new().run();

    //sort::run().unwrap();
    //introduction::run().unwrap();
}