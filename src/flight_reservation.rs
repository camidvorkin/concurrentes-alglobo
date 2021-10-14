use std::string::ToString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FlightReservation {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub hotel: bool,
}

impl ToString for FlightReservation {
    fn to_string(&self) -> String {
        format!("FlightReservation: origin: {}, destination: {}, airline: {}, hotel: {}", self.origin, self.destination, self.airline, self.hotel)
    }
}

impl Clone for FlightReservation {
    fn clone(&self) -> Self {
        FlightReservation {
            origin: self.origin.clone(),
            destination: self.destination.clone(),
            airline: self.airline.clone(),
            hotel: self.hotel,
        }
    }
}

impl FlightReservation {
    // For logging purposes
    pub fn get_flight_code(&self) -> String {
        format!("{}-{}", self.origin, self.destination)
    }

    pub fn get_path(&self) -> String {
        format!("{} -> {}", self.origin, self.destination)
    }
}
