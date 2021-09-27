use std::collections::HashMap;

use crate::airline::Airline;

pub struct AirlineFactory {
    airlines: HashMap<String, Airline>,
}

impl AirlineFactory {
    pub fn new() -> AirlineFactory {
        let airlines = HashMap::<String, Airline>::new();
        AirlineFactory {
            airlines
        }
    }

    pub fn create_airlines(&mut self, airline_names: Vec<Vec<String>>) {
        for airline in airline_names {
            self.airlines.insert(airline[0].clone(), Airline::new(airline[1].parse::<i8>().unwrap().clone()));
        }
    }

    pub fn get(&self, airline_code: &str) -> &Airline {
        match self.airlines.get(airline_code) {
            Some(airline) => airline,
            None => panic!("Airline not found"),
        }
    }
}