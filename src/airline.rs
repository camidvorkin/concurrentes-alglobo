
use std::thread;
use rand::Rng;


pub struct Airline {}


impl Airline {
    pub fn new() -> Airline {
        Airline {

        }
    }

    pub fn reserve_flight(&self) -> bool {
        // thread::sleep(Duration::from_secs(1));
        let rng = rand::thread_rng().gen_bool(0.5);
        return rng;
    }
}