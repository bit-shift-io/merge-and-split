#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split::run;

mod sort;
mod introduction;
mod particle;
mod particle_system;

fn main() {
    //run().unwrap();

    //sort::run().unwrap();
    introduction::run().unwrap();
}