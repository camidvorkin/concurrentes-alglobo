//! Handle airlines config
use std_semaphore::Semaphore;
use std::collections::HashMap;
use std::sync::Arc;
use crate::utils::read_file;

/// Keep track of how many threads can each airline handle
pub type Airlines = HashMap<String, Arc<Semaphore>>;

/// Read from a CSV file with airlines and their max number of concurrent requests as columns and convert it into our Airlines type
pub fn from_file(filename: &str) -> Airlines {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Arc<Semaphore>>::new();
    for airline in airlines {
        airline_map.insert(airline[0].clone(), Arc::new(Semaphore::new(airline[1].parse::<isize>().unwrap())));
    }
    airline_map
}
