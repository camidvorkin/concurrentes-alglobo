#![forbid(unsafe_code)]
mod airlines;
mod flight_reservation;
mod hotel;
mod statsactor;
mod utils;
mod logger;
use actix::prelude::*;

use airlines::{Airline, InfoFlight};

use hotel::Hotel;
use statsactor::{Stat, StatsActor};
use std::collections::HashMap;

use utils::process_flights;

const AIRLINES_FILE: &str = "src/configs/airlines.txt";
const RATE_LIMITING_DEFAULT: usize = 5;

#[actix_rt::main]
async fn main() {
    logger::log_init();
    logger::log("Start Process".to_string());

    let airlines = airlines::from_file(AIRLINES_FILE);
    let addr_hotel = hotel::get_hotel_address(RATE_LIMITING_DEFAULT);
    let addr_statistics = StatsActor {
        sum_time: 0,
        destinations: HashMap::<String, i64>::new(),
    }
    .start();
    
    let mut flights = process_flights("flights.txt");
    let mut flights_for_stats = flights.clone();

    // Create vector of int tuples
    let mut flight_reservations: Vec<(Request<Airline, InfoFlight>, Option<Request<Hotel, InfoFlight>>, std::time::Instant)> = Vec::new();

    for flight_reservation in flights {
        let start_time = std::time::Instant::now();
        let addr_airline = airlines.get(&flight_reservation.airline).unwrap();
        let flight_res = addr_airline.send(InfoFlight {
            flight_reservation: flight_reservation.clone(),
            addr_statistics: addr_statistics.clone(),
        });
        
        let mut hotel_res = Option::None;
        if flight_reservation.hotel {
            hotel_res = Option::Some(addr_hotel.send(InfoFlight {
                flight_reservation: flight_reservation.clone(),
                addr_statistics: addr_statistics.clone(),
            }));
        }
        
        flight_reservations.push((flight_res, hotel_res, start_time));    
    }

    for (flight, hotel, start_time) in flight_reservations {
        let flights_for_stats = flights_for_stats.pop().unwrap();
        let flight = flight.await;
        
        // If Option is not None    
        if let Some(hotel) = hotel {
            let hotel = hotel.await;
        }
        addr_statistics.send(Stat {
            elapsed_time: start_time.elapsed().as_millis(),
            destination: flights_for_stats.get_route()
        }).await;
    }
}
