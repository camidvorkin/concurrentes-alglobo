#![forbid(unsafe_code)]
mod airlines;
mod flight_reservation;
mod hotel;
mod statsactor;
mod utils;
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
    let airlines = airlines::from_file(AIRLINES_FILE);
    let addr_hotel = hotel::get_hotel_address(RATE_LIMITING_DEFAULT);
    let addr_statistics = StatsActor {
        sum_time: 0,
        destinations: HashMap::<String, i64>::new(),
    }
    .start();
    let start_time = std::time::Instant::now();
    // let st_clone = statistics.clone();
    // let addr_statistics = statistics.start();

    let mut flights = process_flights("flights.txt");
    let mut flights2 = flights.clone();

    let mut respuestasvuelo = Vec::<Request<Airline, InfoFlight>>::new();
    let mut respuestashotel = Vec::<Request<Hotel, InfoFlight>>::new();

    for flight_reservation in flights {
        let addr_airline = airlines.get(&flight_reservation.airline).unwrap();
        let flight_res = addr_airline.send(InfoFlight {
            flight_reservation: flight_reservation.clone(),
            addr_statistics: addr_statistics.clone(),
        });
        let hotel_res = addr_hotel.send(InfoFlight {
            flight_reservation: flight_reservation.clone(),
            addr_statistics: addr_statistics.clone(),
        });

        respuestasvuelo.push(flight_res);
        respuestashotel.push(hotel_res);
    }

    while respuestasvuelo.len() > 0 && respuestashotel.len() > 0 {
        let vuelo = respuestasvuelo.pop().unwrap();
        let hotel = respuestashotel.pop().unwrap();
        let flight = flights2.pop().unwrap();

        vuelo.await;
        hotel.await;
    }
}
