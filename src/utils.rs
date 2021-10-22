//! Helper Functions
use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::flight_reservation::FlightReservation;
const IS_HOTEL: &str = "package";

/// Read CSV file and return split content
pub fn read_file(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::<Vec<String>>::new();
    for line in contents.lines() {
        let flight: Vec<String> = line.split(',').map(|x| x.to_string()).collect();
        result.push(flight);
    }
    Ok(result)
}

pub fn process_flights(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).unwrap();
    let mut flights = Vec::<FlightReservation>::new();
    for flight in flights_reservations {
        flights.push(FlightReservation {
            origin: flight[0].clone(),
            destination: flight[1].clone(),
            airline: flight[2].clone(),
            hotel: flight[3].clone() == IS_HOTEL,
        });
    }
    flights
}
