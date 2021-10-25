//! Airline and Hotel servers simulations
use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

/// The minimum amount of millis the request takes
pub const MIN_TIME: u64 = 500;
/// The maximum amount of millis the request takes
pub const MAX_TIME: u64 = 1500;
/// The probability of a request to being successful
pub const RAND_SUCCESSFUL_REQUEST: f64 = 0.8;

/// Simulated request to a hypothetical hotel web server that is always successful
pub fn simulate_hotel() {
    thread::sleep(Duration::from_millis(
        thread_rng().gen_range(MIN_TIME, MAX_TIME),
    ));
}

/// Simulated request to a hypothetical airline web server which can randomly fail
pub fn simulate_airline() -> Result<(), &'static str> {
    thread::sleep(Duration::from_millis(
        thread_rng().gen_range(MIN_TIME, MAX_TIME),
    ));

    match rand::thread_rng().gen_bool(RAND_SUCCESSFUL_REQUEST) {
        true => Ok(()),
        false => Err("Request to airline failed"),
    }
}
