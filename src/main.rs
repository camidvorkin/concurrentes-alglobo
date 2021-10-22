#![forbid(unsafe_code)]
mod airlines;
mod flight_reservation;
mod hotel;
mod logger;
mod statsactor;
mod utils;
use actix::prelude::*;

use airlines::{Airline, InfoFlight};

use flight_reservation::FlightReservation;
use hotel::Hotel;
use statsactor::{Stat, StatsActor};
use std::collections::HashMap;

use utils::process_flights;

const AIRLINES_FILE: &str = "src/configs/airlines.txt";
const RATE_LIMITING_DEFAULT: usize = 2;

#[actix_rt::main]
async fn main() {
    logger::log_init();
    logger::log("Start Process".to_string());

    let airlines = airlines::from_file(AIRLINES_FILE);
    let addr_hotel = hotel::get_hotel_address(RATE_LIMITING_DEFAULT);
    let addr_statistics = StatsActor {
        sum_time: 0,
        destinations: HashMap::<String, i64>::new(),
        flights: HashMap::<i32, i32>::new(),
    }
    .start();

    let mut flights = process_flights("flights.txt");

    let mut flight_reservations: Vec<(
        FlightReservation,
        Request<Airline, InfoFlight>,
        Option<Request<Hotel, InfoFlight>>,
        std::time::Instant,
    )> = Vec::new();

    for flight_reservation in flights {
        let start_time = std::time::Instant::now();
        let addr_airline = airlines.get(&flight_reservation.airline).unwrap();
        let flight_res = addr_airline.send(InfoFlight {
            flight_reservation: flight_reservation.clone(),
            addr_statistics: addr_statistics.clone(),
            start_time: start_time.clone(),
        });

        let mut hotel_res = Option::None;
        if flight_reservation.hotel {
            hotel_res = Option::Some(addr_hotel.send(InfoFlight {
                flight_reservation: flight_reservation.clone(),
                addr_statistics: addr_statistics.clone(),
                start_time: start_time.clone(),
            }));
        }
        flight_reservations.push((flight_reservation, flight_res, hotel_res, start_time));
    }

    for (flight_reservation, flight, hotel, start_time) in flight_reservations {
        let flight = flight.await;

        if let Some(hotel) = hotel {
            let hotel = hotel.await;
        }
    }
}
