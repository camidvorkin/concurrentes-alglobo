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
use std::sync::mpsc::{self, Receiver, Sender};
mod airlines;
mod alglobo;
use common::flight_reservation::FlightReservation;
mod statistics;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use airlines::Airlines;
use common::AIRLINES_FILE;
use statistics::Statistics;

use std::thread;
mod keyboard;
use keyboard::keyboard_listener;
mod logger;
use logger::logger;

/// This is the shared state that will be shared across every thread listening to new requests: the airlines configurations and the universal stats entity
struct AppState {
    airlines: Airlines,
    statistics: Statistics,
    logger_sender: Sender<String>,
}

/// The main function. It starts a thread for the keyboard listener, and it starts the actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let airlines = airlines::from_file(AIRLINES_FILE);
    let statistics = Statistics::new();
    let statistics_keyboard = statistics.clone();
    let statistics_webserver = statistics.clone();

    let (logger_sender, logger_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::Builder::new()
        .name("keyboard".to_string())
        .spawn(move || keyboard_listener(statistics_keyboard))
        .expect("thread creation failed");

    thread::Builder::new()
        .name("logger".to_string())
        .spawn(move || logger(logger_receiver))
        .expect("thread creation failed");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                airlines: airlines.to_owned(),
                statistics: statistics_webserver.to_owned(),
                logger_sender: logger_sender.clone(),
            })
            .service(reservation)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// This documentation isn't showing anywhere on rustdoc :(
#[post("/")]
fn reservation(req: web::Json<FlightReservation>, appstate: web::Data<AppState>) -> HttpResponse {
    let flight: FlightReservation = req.clone();
    let semaphore = appstate.airlines.get(&req.airline);
    match semaphore {
        None => {
            return HttpResponse::NotAcceptable()
                .body("Airline not present on server configuration");
        }
        Some(semaphore) => {
            appstate
                .logger_sender
                .send(format!("[{}] New Request", flight.to_string()))
                .expect("Logger mpsc not receving messages");

            alglobo::reserve(
                flight,
                semaphore.clone(),
                appstate.statistics.clone(),
                appstate.logger_sender.clone(),
            );

            HttpResponse::Ok().finish()
        }
    }
}
