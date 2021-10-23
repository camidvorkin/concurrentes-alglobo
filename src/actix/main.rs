#![forbid(unsafe_code)]
mod airlines;
mod flight;
mod hotel;
mod logger;
mod statsactor;
use actix::prelude::*;

use airlines::Airline;
use common::flight_reservation::FlightReservation;
use common::utils::read_file;
use flight::InfoFlight;
use hotel::Hotel;
use statsactor::{StatsActor, XXX};
use std::collections::HashMap;

const IS_HOTEL: &str = "package";

/// Read CSV file with all the flights requests and return a vector of every FlightReservation
pub fn process_flights(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).unwrap();
    let mut flights = Vec::<FlightReservation>::new();
    let mut i = 1;
    for flight in flights_reservations {
        flights.push(FlightReservation {
            id: i,
            origin: flight[0].clone(),
            destination: flight[1].clone(),
            airline: flight[2].clone(),
            hotel: flight[3].clone() == IS_HOTEL,
        });
        i += 1;
    }
    flights
}

const AIRLINES_FILE: &str = "src/actix/configs/airlines.txt";
const RATE_LIMITING_DEFAULT: usize = 2;

#[actix_rt::main]
async fn main() {
    logger::log_init();
    logger::log("Start Process".to_string());

    // Create the actor system
    let addr_airlines = airlines::from_file(AIRLINES_FILE);
    let addr_hotel = hotel::get_hotel_address(RATE_LIMITING_DEFAULT);
    let addr_statistics = StatsActor {
        sum_time: 0,
        destinations: HashMap::<String, i64>::new(),
        flights: HashMap::<i32, i32>::new(),
    }
    .start();

    // Process the flights reservations
    let flights = process_flights("flights.txt");

    // Vector of futures to wait for the end of the process
    let mut flight_responses: Vec<(
        Request<Airline, InfoFlight>,
        Option<Request<Hotel, InfoFlight>>,
    )> = Vec::new();

    for flight_reservation in flights {
        let start_time = std::time::Instant::now();
        let addr_airline = addr_airlines.get(&flight_reservation.airline).unwrap();
        let info_flight = InfoFlight {
            flight_reservation: flight_reservation.clone(),
            addr_statistics: addr_statistics.clone(),
            start_time: start_time.clone(),
        };

        // Create the future for the airline
        let flight_res = addr_airline.send(info_flight.clone());

        // Create the future for the hotel if the flight is full package, otherwise, create a None
        let mut hotel_res = Option::None;
        if flight_reservation.hotel {
            hotel_res = Option::Some(addr_hotel.send(info_flight));
        }
        flight_responses.push((flight_res, hotel_res));
    }

    for (flight, hotel) in flight_responses {
        let _flight = flight.await;

        if let Some(hotel) = hotel {
            let _hotel = hotel.await;
        }
    }

    let _x = addr_statistics
        .send(XXX {
            s: "Termine".to_string(),
        })
        .await;

    // TODO: Si el ultimo vuelo llega mucho mas tarde que el hotel o viseversa, no se imprime el ultimo stat
    // Hay que encontrar la manera de esperar los try_send
}
