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
mod airline_manager;
mod hotel;
mod info_flight;
mod stats_actor;
use actix::prelude::*;
use airline_manager::NewRequest;
use common::logger::{self, LogLevel};
use common::{flight_reservation, AIRLINES_FILE, TEST_FLIGHTS_FILE};
use hotel::Hotel;
use info_flight::InfoFlight;
use stats_actor::StatsActor;
use std::env;

/// Main function of the actor system
///
/// It reads the CLA flights file (or defaults to a testing purposes one), starts the actors and processes all the requests.
fn main() {
    let system = System::new();
    system.block_on(async {
        logger::init();
        let flights_file = match env::args().nth(1) {
            Some(val) => val,
            None => TEST_FLIGHTS_FILE.to_string(),
        };

        let flights = flight_reservation::from_file(&flights_file);
        logger::log(format!("{} file proccessed", flights_file), LogLevel::TRACE);

        let addr_statistics = StatsActor::new(flights.len()).start();
        logger::log("StatsActor created".to_string(), LogLevel::TRACE);

        let airline_manager = airline_manager::from_file(AIRLINES_FILE, addr_statistics.clone());
        logger::log(
            "Airlines file proccessed and AirlineManager actor created".to_string(),
            LogLevel::TRACE,
        );

        let addr_hotel = Hotel { addr_statistics }.start();
        logger::log("Hotel actor created".to_string(), LogLevel::TRACE);

        for flight_reservation in flights {
            logger::log(format!("{} | START", flight_reservation), LogLevel::INFO);
            let start_time = std::time::Instant::now();
            let info_flight = InfoFlight {
                flight_reservation: flight_reservation.clone(),
                start_time,
                addr_manager: airline_manager.clone(),
                is_retry: false,
            };
            let _airline_res = airline_manager.try_send(NewRequest {
                info_flight: info_flight.clone(),
            });
            let _hotel_res = match flight_reservation.hotel {
                true => Some(addr_hotel.try_send(info_flight.clone())),
                false => None,
            };
        }
    });

    system.run().unwrap();
}
