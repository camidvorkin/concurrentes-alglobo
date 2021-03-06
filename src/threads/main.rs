//! AlGlobo - Simple HTTP server for reserving a flight
//! ---
//! This server receives POST calls from the client and attempts to make a reservation to the corresponding airline
//!
//! Start the server with `cargo run --bin threads`
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
//! If a (simulated) airline request isn't succesful, we retry that request after N seconds have passed. This N can be configured under the RETRY_SECONDS environment variable
//!
//! The supported airlines and the number of requests they can handle simultaneously can be configured in the `configs/airlines.txt` file
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

use actix_web::rt::System;
use common::flight_reservation::FlightReservation;
mod statistics;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use airlines::Airlines;
use common::logger::LogLevel;
use common::logger::{self, LoggerMsg};
use common::AIRLINES_FILE;
use statistics::Statistics;

use std::thread;
mod keyboard;
use keyboard::keyboard_loop;

#[cfg(test)]
mod test;

/// This is the shared state that will be shared across every thread listening to new requests
///
/// [Actix Web State](https://actix.rs/docs/application/#state)
struct AppState {
    /// Each airline supported by the app, with the number of requests that can be handled simultaneously
    airlines: Airlines,
    /// The flights stats
    statistics: Statistics,
    /// A reference to the logger, so that each thread can log their own actions, in a safe way
    logger_sender: Sender<LoggerMsg>,
}

/// The main function. It spawns different threads and starts up the HTTP server
///
/// This function spawns a thread for the logger, a thread for the HTTP server (which in turn, handled by actix web, spawns N workers to receive requests) and listens for keyboard events
#[actix_web::main]
async fn main() {
    let (logger_sender, logger_receiver): (Sender<LoggerMsg>, Receiver<LoggerMsg>) =
        mpsc::channel();
    let logger_sender_webserver = logger_sender.clone();

    // We create a mpsc for the HTTP Server, so that we can run it on its own thread and then gracefully shut it down.
    // Without this mpsc we woudln't have any way to reference the server created, and we wouldn't be able to shut it down
    // This is also the reason why we run the server on a different thread, so that we can send it through the mpsc instead of having a big `await` on our main and no way to call a stop() function. All of this is explained on actix-web's docs
    // https://actix.rs/docs/server/#the-http-server
    let (server_sender, server_receiver) = mpsc::channel();

    let _logger_thread = thread::Builder::new()
        .name("logger".to_string())
        .spawn(move || {
            logger::init();

            while let Ok((msg, loglevel)) = logger_receiver.recv() {
                logger::log(msg, loglevel);
                if let LogLevel::FINISH = loglevel {
                    break;
                }
            }
        })
        .expect("thread creation failed");

    let airlines = airlines::from_file(AIRLINES_FILE);
    logger_sender
        .send(("Airlines CSV files processed".to_string(), LogLevel::TRACE))
        .expect("Logger mpsc not receving messages");

    let statistics = Statistics::new();
    let statistics_webserver = statistics.clone();

    // This thread gets terminated when calling server.stop()
    // https://docs.rs/actix-web/0.3.3/actix_web/server/struct.StopServer.html
    let _server_thread = thread::Builder::new()
        .name("http-server".to_string())
        .spawn(move || {
            let sys = System::new("http-server");

            logger_sender_webserver
                .send(("Server thread started".to_string(), LogLevel::TRACE))
                .expect("Logger mpsc not receving messages");

            let srv = HttpServer::new(move || {
                App::new()
                    .data(AppState {
                        airlines: airlines.to_owned(),
                        statistics: statistics_webserver.to_owned(),
                        logger_sender: logger_sender_webserver.to_owned(),
                    })
                    .service(reservation)
            })
            .bind(("127.0.0.1", 8080))
            .expect("Server couldn't start")
            .run();

            let _ = server_sender.send(srv);
            sys.run()
        })
        .expect("thread creation failed");

    logger_sender
        .send(("Keyboard started listening".to_string(), LogLevel::TRACE))
        .expect("Logger mpsc not receving messages");

    // Reference to the server, so that we can shut it down
    let srv = server_receiver
        .recv()
        .expect("Couldn't receive server ref through mpsc");

    // This is an infinite loop that only exits on a QUIT command
    // Anything after this line is part of the graceful shutdown
    keyboard_loop(statistics, &logger_sender);

    // We stop the server, which joins the server thread
    // This is a graceful shutdown, so any request still in place will be completed before shutdown
    srv.stop(true).await;

    // We send a loglevel::FINISH message, which breaks the logger infinite loop
    logger_sender
        .send(("Shut down server".to_string(), LogLevel::FINISH))
        .expect("Logger mpsc not receving messages");
}

/// This documentation isn't showing anywhere on rustdoc :(
#[post("/")]
fn reservation(req: web::Json<FlightReservation>, appstate: web::Data<AppState>) -> HttpResponse {
    appstate
        .logger_sender
        .send((format!("POST / -- {:?}", req), LogLevel::INFO))
        .expect("Logger mpsc not receving messages");

    let flight: FlightReservation = req.to_owned();
    let semaphore = appstate.airlines.get(&req.airline);
    match semaphore {
        None => {
            appstate
                .logger_sender
                .send((
                    format!("{} | BAD REQUEST | Airport not present", flight.to_string()),
                    LogLevel::INFO,
                ))
                .expect("Logger mpsc not receving messages");

            HttpResponse::NotAcceptable().body("Airline not present on server configuration")
        }
        Some(semaphore) => {
            appstate
                .logger_sender
                .send((format!("{} | START", flight), LogLevel::INFO))
                .expect("Logger mpsc not receving messages");

            alglobo::reserve(
                flight,
                semaphore.to_owned(),
                appstate.statistics.to_owned(),
                appstate.logger_sender.to_owned(),
            );

            HttpResponse::Ok().finish()
        }
    }
}
