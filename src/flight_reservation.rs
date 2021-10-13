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
        return format!("FlightReservation: origin: {}, destination: {}, airline: {}, hotel: {}", self.origin, self.destination, self.airline, self.hotel);
    }
}

impl Clone for FlightReservation {
    fn clone(&self) -> Self {
        return FlightReservation {
            origin: self.origin.clone(),
            destination: self.destination.clone(),
            airline: self.airline.clone(),
            hotel: self.hotel,
        };
    }
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

    pub fn get_airline(&self) -> String {
        return self.airline.clone();
    }

    pub fn get_hotel(&self) -> bool {
        return self.hotel.clone();
    }

    // For logging purposes
    pub fn get_flight_code(&self) -> String {
        return format!("{}-{}", self.origin, self.destination);
    }

    pub fn get_path(&self) -> String {
        return format!("Origin: {} -> Destination: {}", self.origin, self.destination);
    }
}
