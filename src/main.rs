//! AlGlobo - Simple HTTP server for reserving a flight
//! ---
//! This server receives POST calls from the client and attempts to make a reservation to the corresponding airline
//!
//! Start the server with `cargo run` or even `cargo run airlines.txt` to specify the airlines file (specified on the `airlines.rs` module)
//!
//! The requests are simple POSTs to the route `/` with a JSON body formatted as
//! ```json
//!  {
//!   "origin": "EZE", // Origin airport
//!   "destination": "JFK", // Destination airport
//!   "airline": "AA", // Airline code, which of course should be present on our config file
//!   "hotel": true // If the client wants to send a package, it must go through the hotel server
//!  }
//! ```
//!
//! This example could be sent with curl as: `curl -i -d '{"origin":"EZE", "destination":"JFK", "airline":"AA", "hotel":true}' -H "Content-Type: application/json" -X POST http://localhost:8080/`
//!
//! ---
//!
//! Response Status of the actix-web server:
//! * `200` if the request was successful -> this doesn't mean that the flight reservation was successful! It just means that the communication was successful
//! * `406` if the JSON was valid, but the airline is not supported by our server (not present on the config file)
//! * `400` if the request JSON is not valid
//!
//! ---
//!
//! The server has a thread always listening to keyboard events. If the user presses `s` the server will show the flight stats, and if the user presses `q` the server will gracefully exit.

#![forbid(unsafe_code)]
mod airlines;
mod hotel;
mod flight_reservation;
mod informe;
mod statsactor;
mod utils;
use crate::flight_reservation::FlightReservation;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use actix::{Actor, System};
use airlines::{Airlines, InfoFlight};
use hotel::{Hotel, InfoPackage};
use statsactor::{StatsActor, Stat};
use actix::prelude::*;
use std::collections::HashMap;

/// This can be either a CLA or this default value
const AIRLINES_FILE: &str = "src/configs/airlines.txt";
const RATE_LIMITING_DEFAULT: usize = 5;

/// Possible command strings that trigger the exit action
// const QUIT_COMMANDS: [&str; 2] = ["Q", "QUIT"];

/// Possible command strings that trigger the show stats action
// const STAT_COMMANDS: [&str; 2] = ["S", "STATS"];

/// This is the shared state that will be shared across every thread listening to new requests: the airlines configurations and the universal stats entity
struct AppState {
    airlines: Airlines,
    hotel: Addr<Hotel>,
    statistics: Addr<StatsActor>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let system = System::new();

    let airlines = airlines::from_file(AIRLINES_FILE);
    let hotel = hotel::get_hotel_address(RATE_LIMITING_DEFAULT);
    let statistics = StatsActor { sum_time: 0, destinations: HashMap::<String, i64>::new()};
    let st_clone = statistics.clone();
    let addr_statistics = statistics.start();


    let a = HttpServer::new(move || {
        App::new()
            .data(AppState {
                airlines: airlines.to_owned(),
                hotel: hotel.to_owned(),
                statistics: addr_statistics.to_owned()
            })
            .service(reservation)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    st_clone.print_stat();
    system.run().unwrap();
    a
}

/// This documentation isn't showing anywhere on rustdoc :(
#[post("/")]
async fn reservation(req: web::Json<FlightReservation>, appstate: web::Data<AppState>) -> HttpResponse {
    let flight: FlightReservation = req.clone();
    let package: FlightReservation = req.clone();
    let flights_for_stats: FlightReservation = req.clone();
    let start_time = std::time::Instant::now();

    // Todo: logger
    let s = format!("[{}] New Request", flight.to_string());
    print!("{}", s);

    // Todo validate if not in hashmap
    let addr_airline = appstate.airlines.get(&flight.airline.to_string()).unwrap();
    let addr_statistics = appstate.statistics.clone();
    let addr_hotel = appstate.hotel.clone();
    
    let flightOk = addr_airline.send(InfoFlight { flight_reservation: flight });
    let hotelOk = addr_hotel.send(InfoPackage { route: package.get_route().to_string() });
    
    flightOk.await;
    hotelOk.await;
    addr_statistics.send(Stat {elapsed_time: start_time.elapsed().as_millis(), destination : flights_for_stats.get_route().to_string()}).await;
    
    // appstate.statistics.send(Stat {name: String::from("aaaaa")}).await;

    HttpResponse::Ok().finish()
}
