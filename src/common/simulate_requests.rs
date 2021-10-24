use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

pub const MIN_TIME: u64 = 500;
pub const MAX_TIME: u64 = 1500;
pub const RAND_HOTEL: f64 = 0.8;

/// Simulated request to a hypothetical hotel web server
pub fn simulate_hotel() -> () {
    thread::sleep(Duration::from_millis(thread_rng().gen_range(MIN_TIME, MAX_TIME)));
}

/// Simulated request to a hypothetical airline web server
pub fn simulate_airline() -> Result<(), &'static str> {
    thread::sleep(Duration::from_millis(thread_rng().gen_range(MIN_TIME, MAX_TIME)));

    match rand::thread_rng().gen_bool(RAND_HOTEL) {
        true => Ok(()),
        false => Err("Request to airline failed"),
    }
}
