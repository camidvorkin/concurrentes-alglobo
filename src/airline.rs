
use std::thread;
use rand::Rng;
use std::time::Duration;
use std::clone::Clone;
use std::marker::Copy;

pub struct Airline {
}

impl Copy for Airline {}

impl Clone for Airline {
    fn clone(&self) -> Self {
        Airline {
        }
    }
}



impl Airline {
    pub fn new() -> Airline {
        Airline {
        }
    }

    pub fn reseve(&self) -> bool {
        thread::sleep(Duration::from_secs(1));
        let rng = rand::thread_rng().gen_bool(0.5);
        return rng;
    }
}