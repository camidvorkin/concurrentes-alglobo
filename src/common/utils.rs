//! Helper Functions
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use rand::Rng;

/// Read CSV file and return split content under a Rust Vec
///
/// Helpful for the flights and airlines files
pub fn read_file(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::<Vec<String>>::new();
    for line in contents.lines() {
        let flight: Vec<String> = line.split(',').map(|x| x.to_string()).collect();
        result.push(flight);
    }
    Ok(result)
}

/// If the user doesn't set the ENVVAR `RETRY_SECONDS` we default to this value
pub const DEFAULT_RETRY_SECONDS: u64 = 2;

/// Returns either the ENV VAR value or the default one
pub fn get_retry_seconds() -> u64 {
    match env::var("RETRY_SECONDS") {
        Ok(val) => val
            .parse::<u64>()
            .expect("Couldn't parse RETRY_SECONDS env var"),
        Err(_) => DEFAULT_RETRY_SECONDS,
    }
}

/// The probability of a request to being successful
pub const SUCCESFUL_RATE: f64 = 0.8;

/// Throws a coin and returns a Result
pub fn toin_coss() -> Result<(), &'static str> {
    match rand::thread_rng().gen_bool(SUCCESFUL_RATE) {
        true => Ok(()),
        false => Err("Request to airline failed"),
    }
}
