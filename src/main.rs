#![allow(dead_code, unused_variables, unused_imports)]
#![feature(test)]

use merge_and_split::run;
mod sort;
mod introduction;

pub mod particles;

pub mod math;


fn main() {
    //run().unwrap();

    //sort::run().unwrap();
    introduction::run().unwrap();
}