use std_semaphore::Semaphore;
use std::collections::HashMap;
use std::sync::Arc;
use crate::utils::read_file;

pub type Airlines = HashMap<String, Arc<Semaphore>>;

pub fn from_file(filename: &str) -> Airlines {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Arc<Semaphore>>::new();
    for airline in airlines {
        airline_map.insert(airline[0].clone(), Arc::new(Semaphore::new(airline[1].parse::<isize>().unwrap())));
    }
    return airline_map;
}
