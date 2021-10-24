#![forbid(unsafe_code)]
mod airline;
mod hotel;
mod info_flight;
mod stats_actor;
use actix::prelude::*;

use airline::Airline;
use common::logger::{self, LogLevel};

use common::{flight_reservation, AIRLINES_FILE, TEST_FLIGHTS_FILE};
use hotel::Hotel;
use info_flight::InfoFlight;
use stats_actor::{FinishMessage, StatsActor};

use std::env;

#[actix_rt::main]
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

    // Vector of futures to wait for the end of the process
    type AirlineReq = Request<Airline, InfoFlight>;
    type HotelReq = Request<Hotel, InfoFlight>;
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
                    format!(
                        "{} | BAD REQUEST | Airport not present",
                        flight_reservation.to_string()
                    ),
                    LogLevel::INFO,
                );
                continue;
            }
            Some(val) => {
                logger::log(
                    format!("{} | START", flight_reservation.to_string()),
                    LogLevel::INFO,
                );
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

    for (flight, hotel) in responses {
        let _flight = flight.await;

        if let Some(hotel) = hotel {
            let _hotel = hotel.await;
        }
    }

    let _finish = addr_statistics.send(FinishMessage).await;
    logger::log("Shut down".to_string(), LogLevel::FINISH);
}
