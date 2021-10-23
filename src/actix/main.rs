#![forbid(unsafe_code)]
mod airline;
mod hotel;
mod info_flight;
mod stats_actor;
use actix::prelude::*;

use airline::Airline;
use common::logger;

use common::{flight_reservation, AIRLINES_FILE, TEST_FLIGHTS_FILE};
use hotel::Hotel;
use info_flight::InfoFlight;
use stats_actor::{FinishMessage, StatsActor};
use std::collections::HashMap;
use std::env;

#[actix_rt::main]
async fn main() {
    logger::init();

    let flights_file = match env::args().nth(1) {
        Some(val) => val,
        None => TEST_FLIGHTS_FILE.to_string(),
    };
    let flights = flight_reservation::from_file(&flights_file);

    let addr_statistics = StatsActor::new().start();
    let addr_statistics_hotel = addr_statistics.clone();
    let addr_statistics_airline = addr_statistics.clone();

    let addr_airlines = airline::from_file(AIRLINES_FILE, addr_statistics_airline);

    let hotel_count = flights.iter().filter(|f| f.hotel).count();
    let addr_hotel = SyncArbiter::start(hotel_count, move || Hotel {
        addr_statistics: addr_statistics_hotel.to_owned(),
    });

    // Vector of futures to wait for the end of the process
    let mut responses: Vec<(
        Request<Airline, InfoFlight>,
        Option<Request<Hotel, InfoFlight>>,
    )> = Vec::new();

    for flight_reservation in flights {
        let start_time = std::time::Instant::now();
        let info_flight = InfoFlight {
            flight_reservation: flight_reservation.clone(),
            start_time,
        };

        let addr_airline = match addr_airlines.get(&flight_reservation.airline) {
            None => {
                logger::log(format!(
                    "Received flight with airline not present in config: {}",
                    flight_reservation.airline
                ));
                continue;
            }
            Some(val) => val,
        };

        let flight_res = addr_airline.send(info_flight.clone());
        let hotel_res = match flight_reservation.hotel {
            true => Some(addr_hotel.send(info_flight)),
            false => None,
        };

        responses.push((flight_res, hotel_res));
    }

    for (flight, hotel) in responses {
        let _flight = flight.await;

        if let Some(hotel) = hotel {
            let _hotel = hotel.await;
        }
    }

    let _finish = addr_statistics.send(FinishMessage).await;
}
