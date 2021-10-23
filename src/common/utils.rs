//! Helper Functions
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// Read CSV file and return split content
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
pub const DEFAULT_RETRY_SECONDS: u64 = 5;

pub fn get_retry_seconds() -> u64 {
    match env::var("RETRY_SECONDS") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_) => DEFAULT_RETRY_SECONDS,
    }
}
