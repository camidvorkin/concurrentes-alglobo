
use crate::airline::Airline;

pub struct FlightReservation {
    origin: String,
    destination: String,
    airline: String,
    hotel: bool,
}

impl FlightReservation {
    pub fn new(origin: String, destination: String, airline: String, hotel: bool) -> FlightReservation {
        FlightReservation {
            origin,
            destination,
            airline,
            hotel
        }
    }

    pub fn send_to_airline(&self, airline: &Airline) {
        if !airline.reserve_flight() {
            print!("Flight reservation failed\n");
            self.send_to_airline(airline);
        } else {
            print!("Flight reservation successful\n");
        }
    }

    pub fn get_airline(&self) -> String {
        return self.airline.clone();
    }
}

pub enum PackageType {
    HOTEL,
    COMPLETE,
}