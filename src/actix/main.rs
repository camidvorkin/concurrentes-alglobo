//! AlGlobo - Actor system to process flights
//! ---
//! This program reads a CSV file with flights data and starts to simulate their reservations
//!
//! The CSV file can be either the first argument of the launch command, or the testing default one
//!
//! Start the program with `cargo run --bin actix <flightsfile.txt>`
//!
//! The CSV has the origin, destination, airline and a boolean that indicates if the flight is a hotel one or not as columns
//!
//! The airlines in the flights file must be configured in the airlines file, under the configs dir. This CSV has the airline name and the number of simultaneous flights they can handle
//!
//! Every N flights processed, the program will print the current stats of the system. This variable can be configured under the stats actor file
mod airline;
mod hotel;
mod info_flight;
mod stats_actor;
use actix::prelude::*;

use airline::Airline;
use common::logger::{self, LogLevel};

use common::{flight_reservation, AIRLINES_FILE, TEST_FLIGHTS_FILE};
use flight_reservation::FlightReservation;
use hotel::Hotel;
use info_flight::InfoFlight;
use stats_actor::{FinishMessage, StatsActor};
use std::collections::HashMap;
use std::env;

/// Vector of futures to await for the end of the airline reqs
type AirlineReq = Request<Airline, InfoFlight>;
/// Vector of futures to await for the end of the hotel reqs
type HotelReq = Request<Hotel, InfoFlight>;

// #[cfg(test)]
// mod test;

/// Make a flight reservation by sending it to the airline and hotel actors
pub async fn reserve(
    flights: Vec<FlightReservation>,
    addr_airlines: HashMap<String, Addr<Airline>>,
    addr_hotel: Addr<Hotel>,
) -> Vec<(AirlineReq, Option<HotelReq>)> {
    let mut responses: Vec<(AirlineReq, Option<HotelReq>)> = Vec::new();

    for flight_reservation in flights {
        let start_time = std::time::Instant::now();
        let info_flight = InfoFlight {
            flight_reservation: flight_reservation.clone(),
            start_time,
        };
        let addr_airline = match addr_airlines.get(&flight_reservation.airline) {
            None => {
                logger::log(
                    format!("{} | BAD REQUEST | Airport not present", flight_reservation),
                    LogLevel::INFO,
                );
                continue;
            }
            Some(val) => {
                logger::log(format!("{} | START", flight_reservation,), LogLevel::INFO);
                val
            }
        };

        let flight_res = addr_airline.send(info_flight.clone());
        let hotel_res = match flight_reservation.hotel {
            true => Some(addr_hotel.send(info_flight)),
            false => None,
        };

        responses.push((flight_res, hotel_res));
    }
    responses
}

#[actix_rt::main]
/// Main function of the actor system
///
/// It reads the CLA flights file (or defaults to a testing purposes one), starts the actors and processes all the requests. Then, it gracefully shuts down the system
async fn main() {
    logger::init();

    let flights_file = match env::args().nth(1) {
        Some(val) => val,
        None => TEST_FLIGHTS_FILE.to_string(),
    };

    let flights = flight_reservation::from_file(&flights_file);
    logger::log(format!("{} file proccessed", flights_file), LogLevel::TRACE);

    let addr_statistics = StatsActor::new().start();
    logger::log("StatsActor created".to_string(), LogLevel::TRACE);
    let addr_statistics_hotel = addr_statistics.clone();
    let addr_statistics_airline = addr_statistics.clone();

    let addr_airlines = airline::from_file(AIRLINES_FILE, addr_statistics_airline);
    logger::log("Airlines file proccessed".to_string(), LogLevel::TRACE);

    let hotel_count = flights.iter().filter(|f| f.hotel).count();
    let addr_hotel = SyncArbiter::start(hotel_count, move || Hotel {
        addr_statistics: addr_statistics_hotel.to_owned(),
    });
    logger::log(
        format!("Hotel SyncArbier with {} count created", hotel_count),
        LogLevel::TRACE,
    );
    let responses: Vec<(AirlineReq, Option<HotelReq>)> =
        reserve(flights, addr_airlines, addr_hotel).await;

    for (flight, hotel) in responses {
        let _flight = flight.await;

        if let Some(hotel) = hotel {
            let _hotel = hotel.await;
        }
    }

    let _finish = addr_statistics.send(FinishMessage).await;
    logger::log("Shut down".to_string(), LogLevel::FINISH);
}
