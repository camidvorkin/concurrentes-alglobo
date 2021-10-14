//! AlGlobo - Simple HTTP server for reserving a flight
//! ---
//! This server receives POST calls from the client and attempts to make a reservation to the corresponding airline
//!
//! Start the server with `cargo run` or even `cargo run airlines.txt` to specify the airlines file
//! TODO
use std::env;
mod flight_reservation;
mod statistics;
mod utils;
mod alglobo;
mod informe;
mod airlines;
use airlines::Airlines;
use actix_web::{web, App, HttpResponse, HttpServer, post};
use statistics::Statistics;
use crate::flight_reservation::FlightReservation;
use std::{io, thread};
use std::io::prelude::*;

const AIRLINES_FILE: &str = "src/configs/airlines.txt";
const QUIT_COMMANDS: [&'static str; 2] = ["Q", "QUIT"];
const STAT_COMMANDS: [&'static str; 2] = ["S", "STATS"];

/// TODO
struct AppState {
    /// TODO
    airlines: Airlines,
    /// TODO
    statistics: Statistics,
}

fn keyboard_listener(statistics: Statistics) {
    println!("At any time press S to query for stats or Q to gracefully exit \n");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let input = &*line.trim().to_uppercase();
                if QUIT_COMMANDS.contains(&input) {
                    // TODO: como puedo matar a un thread desde si mismo?
                    // o mandar una señal a main para que lo mate
                    println!("quit");
                    break;
                }
                else if STAT_COMMANDS.contains(&input) {
                    println!("Operational Stats \n\
                              * Completed Flights: {} \n\
                              * Total Waiting Time: {} \n\
                              * Avg Response time: {:.2}", statistics.get_total_count(),
                                                           statistics.get_sum_time(),
                                                           statistics.get_avg_time());

                    let top_routes = statistics.get_top_destinations(10);
                    if top_routes.len() > 0 {
                        println!("\nTop {} most solicited routes", top_routes.len());
                        for (k, v) in top_routes {
                            println!("* {} ({} flights)", k, v);
                        }
                    }


                }
            },
            Err(_) => panic!("Failed to read stdin")
        }
    };
}

/// TODO
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // The Airlines config file is either a CLA or hardcoded in our configs directory
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => AIRLINES_FILE.to_string(),
    };

    let statistics = Statistics::new();
    let statistics_keyboard = statistics.clone();
    let statistics_webserver = statistics.clone();

    thread::spawn(move || {
        keyboard_listener(statistics_keyboard.to_owned());
    });


    HttpServer::new(move || {
        App::new()
            .data(AppState {
                airlines: airlines::from_file(&filename_airline),
                statistics: statistics_webserver.to_owned(),
            })
            .service(reservation)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}

/// TODO
#[post("/")]
fn reservation(req: web::Json<FlightReservation>, appstate: web::Data<AppState>) -> HttpResponse {
    let semaphore = appstate.airlines.get(&req.airline);
    match semaphore {
        None => {
            return HttpResponse::NotAcceptable().body("Airline not present on server configuration");
        },
        Some(_) => (),
    };

    let flight: FlightReservation = req.clone();
    println!("New Request! {}", flight.to_string());
    let reservation = alglobo::reserve(flight,
                                       semaphore.unwrap().clone(),
                                       appstate.statistics.clone());
    reservation.join().unwrap();
    HttpResponse::Ok().finish()
}
