use std::collections::HashMap;

use crate::airline::Airline;

pub struct AirlineFactory {
    airlines: HashMap<String, Airline>,
}

impl AirlineFactory {
    pub fn new() -> AirlineFactory {
        let mut airlines = HashMap::<String, Airline>::new();
        AirlineFactory {
            airlines
        }
    }

    pub fn create_airlines(&mut self) {
        self.airlines.insert("AA".to_string(), Airline::new());
        self.airlines.insert("UA".to_string(), Airline::new());
        self.airlines.insert("DL".to_string(), Airline::new());
        self.airlines.insert("AS".to_string(), Airline::new());
        self.airlines.insert("F9".to_string(), Airline::new());
        self.airlines.insert("B6".to_string(), Airline::new());
        self.airlines.insert("LATAM".to_string(), Airline::new());
    }

    pub fn get(&self, airline_code: &str) -> &Airline {
        match self.airlines.get(airline_code) {
            Some(airline) => airline,
            None => panic!("Airline not found"),
        }
    }
}