
use std::thread;
use rand::Rng;
use std::time::Duration;
use std::clone::Clone;
use std::marker::Copy;

#[derive(Copy, Clone)]
pub struct Airline {
    capacity: i8,
}


impl Airline {
    pub fn new(capacity: i8) -> Airline {
        Airline {
            capacity,
        }
    }

    pub fn reseve(&self) -> bool {
        thread::sleep(Duration::from_secs(1));
        let rng = rand::thread_rng().gen_bool(0.5);
        return rng;
    }

    pub fn get_airline_capacity(&self) -> i8 {
        self.capacity.clone()
    }
}