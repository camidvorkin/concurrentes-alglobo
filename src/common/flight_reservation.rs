//! Flight Reservations Struct
use crate::utils::read_file;
use serde::Deserialize;
use std::fmt;
/// Struct
///
/// This struct can be deserialized from a JSON (thanks to serde), making it easier to use, because we now can receive it from the web request in a JSON form
#[derive(Deserialize, Debug)]
pub struct FlightReservation {
    #[serde(skip)]
    pub id: i32,
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub hotel: bool,
}

impl fmt::Display for FlightReservation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("{}->{}", self.origin, self.destination);
        match self.hotel {
            true => string += &format!("({}+)", self.airline),
            false => string += &format!("({})", self.airline),
        }
        write!(f, "{}", string)
    }
}

impl Clone for FlightReservation {
    fn clone(&self) -> Self {
        FlightReservation {
            id: self.id,
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

/// Read CSV file with all the flights requests and return a vector of every FlightReservation
pub fn from_file(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).expect("Couldn't read flights file");
    let mut flights = Vec::<FlightReservation>::new();
    for (i, flight) in flights_reservations.iter().enumerate() {
        flights.push(FlightReservation {
            id: i as i32,
            origin: flight[0].to_owned(),
            destination: flight[1].to_owned(),
            airline: flight[2].to_owned(),
            hotel: flight[3] == "true",
        });
    }
    flights
}
