use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

/// Simulated request to an hypothetical hotel web server
pub fn simulate_hotel() -> Result<(), &'static str> {
    thread::sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
    Ok(())
}

/// Simulated request to an hypothetical airline web server
pub fn simulate_airline() -> Result<(), &'static str> {
    thread::sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));

    match rand::thread_rng().gen_bool(0.8) {
        true => Ok(()),
        false => Err("Request to airline failed"),
    }
}
