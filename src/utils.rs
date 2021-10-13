use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::flight_reservation::FlightReservation;
use std_semaphore::Semaphore;
use std::collections::HashMap;
use std::sync::Arc;

const PACKAGE: &str = "package";
pub const FLIGHT_RESERVATION_FILE: &str = "test/test.txt";
pub const AIRLINES_FILE: &str = "src/configs/airlines.txt";

// Read file and return split content
fn read_file(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::<Vec<String>>::new();
    for line in contents.lines() {
        let flight: Vec<String>  = line.split(",").map(|x| x.to_string()).collect();
        result.push(flight);
      } 
    Ok(result)
}

// Process Airlines and set rate limiting to Semaphore
pub fn process_airlines(filename: &str) -> HashMap<String, Arc<Semaphore>> {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Arc<Semaphore>>::new();
    for airline in airlines {
        airline_map.insert(airline[0].clone(), Arc::new(Semaphore::new(airline[1].parse::<isize>().unwrap())));
    }
    return airline_map;
}

// Process Flights to set FlightReservation struct
pub fn process_flights(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).unwrap();
    let mut flights = Vec::<FlightReservation>::new();
    for flight in flights_reservations {
        flights.push(FlightReservation::new(flight[0].clone(), flight[1].clone(), flight[2].clone(), flight[3].clone() == PACKAGE))
    }
    flights
}