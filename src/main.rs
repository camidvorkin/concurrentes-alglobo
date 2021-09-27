use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;

mod flight_reservation;
mod airline_factory;
mod airline;
mod hotel;
use crate::flight_reservation::FlightReservation;
use crate::airline_factory::AirlineFactory;
use crate::hotel::Hotel;

const PACKAGE: &str = "package";

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

fn process_airlines(filename: &str) -> AirlineFactory{
    let airlines = read_file(filename).unwrap();
    let mut airline_factory:AirlineFactory = AirlineFactory::new();
    airline_factory.create_airlines(airlines);
    return airline_factory;
}

fn process_flights(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).unwrap();
    let mut flights = Vec::<FlightReservation>::new();
    for flight in flights_reservations {
        flights.push(FlightReservation::new(flight[0].clone(), flight[1].clone(), flight[2].clone(), flight[3].clone() != PACKAGE))
    }
    flights
}

fn main() {
    println!("Hello, world and d*!");

    let hotel:Hotel = Hotel::new();

    // Process airlines
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => "src/configs/airlines.txt".to_string(),
    };
    let airline_factory = process_airlines(&filename_airline);

    // Process flight reservations
    let filename = match env::args().nth(2) {
        Some(val) => val,
        None => "test/test.txt".to_string(),
    };

    let flights = process_flights(&filename);
    for flight_reservation in flights {
        flight_reservation.reserve(airline_factory.get(&flight_reservation.get_airline()), hotel);
    }
}
