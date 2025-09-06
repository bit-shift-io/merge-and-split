#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split::run;
mod sort;
mod introduction;

pub mod math;
pub mod particle;
pub mod particle_system;
pub mod operation;
pub mod operation_merge;
pub mod operation_move;

fn main() {
    //run().unwrap();

    //sort::run().unwrap();
    introduction::run().unwrap();
}