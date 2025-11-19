use rand_seeder::{Seeder, SipHasher};
use rand_pcg::Pcg64;

use chrono::Utc;
use now::DateTimeNow;

/// Random number generator
pub struct Random {
}

impl Random {
    pub fn seed_from_beginning_of_week() -> Pcg64 {
        let time = Utc::now(); 
        let beginning_of_week = time.beginning_of_week();
        let rng: Pcg64 = Seeder::from(beginning_of_week).into_rng();
        rng
    }

    pub fn seed_from_beginning_of_day() -> Pcg64 {
        let time = Utc::now(); 
        let beginning_of_day = time.beginning_of_day();
        let rng: Pcg64 = Seeder::from(beginning_of_day).into_rng();
        rng
    }

    pub fn seed_from_now() -> Pcg64 {
        let rng: Pcg64 = Seeder::from(Utc::now()).into_rng();
        rng
    }
}
