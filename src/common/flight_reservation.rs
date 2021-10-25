//! Flight Reservations Struct
use crate::utils::read_file;
use serde::Deserialize;
use std::fmt;
/// Struct
///
/// This struct can be deserialized from a JSON (thanks to serde), making it easier to use, because we now can receive it from the web request in a JSON form
#[derive(Deserialize, Debug)]
pub struct FlightReservation {
    /// The id is set by the program, not the request
    #[serde(skip)]
    pub id: i32,
    /// Origin airport
    pub origin: String,
    /// Destination airport
    pub destination: String,
    /// Airline
    pub airline: String,
    /// True if the flight is a package, false if it's just a flight
    pub hotel: bool,
}

/// We want the struct to be easily loggable
impl fmt::Display for FlightReservation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("{}->{}", self.origin, self.destination);
        match self.hotel {
            true => string += &format!(" -- {} -- hotel ", self.airline),
            false => string += &format!(" -- {} -- flight", self.airline),
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
    /// Returns the flights route (origin + destination)
    pub fn get_route(&self) -> String {
        format!("{} -> {}", self.origin, self.destination)
    }
}

/// Reads the CSV file with all the flights requests and returns a vector of every FlightReservation
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
