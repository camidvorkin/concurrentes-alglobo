//! Flight Reservations Struct
use actix::Message;
use serde::Deserialize;
use std::string::ToString;

/// Struct
///
/// This struct can be deserialized from a JSON (thanks to serde), making it easier to use, because we now can receive it from the web request in a JSON form
#[derive(Deserialize)]
pub struct FlightReservation {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub hotel: bool,
}

impl Message for FlightReservation {
    type Result = i32;
}

impl ToString for FlightReservation {
    fn to_string(&self) -> String {
        let mut string = format!("{}->{}", self.origin, self.destination);
        if self.hotel {
            string += &format!(" ({}+)", self.airline)
        } else {
            string += &format!(" ({})", self.airline)
        }
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
