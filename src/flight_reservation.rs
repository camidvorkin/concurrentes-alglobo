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
        let mut string = format!("{} by {}", self.get_route(), self.airline);
        if self.hotel { string += " (hotel)"} ;
        string
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
    pub fn get_route(&self) -> String {
        format!("{} -> {}", self.origin, self.destination)
    }
}
