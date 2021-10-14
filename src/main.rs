//! AlGlobo - Simple HTTP server for reserving a flight
//! ---
//! This server receives POST calls from the client and attempts to make a reservation to the corresponding airline
//!
//! Start the server with `cargo run` or even `cargo run airlines.txt` to specify the airlines file
//! TODO
use std::{collections::HashMap, env};
mod flight_reservation;
mod statistics;
mod utils;
mod alglobo;
mod informe;
mod airlines;
use airlines::Airlines;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, post};
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

fn keyboard_listener(statistics_keyboard: Statistics) {
    println!("At any time press S to query for stats or Q to gracefully exit");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let input = &*line.trim().to_uppercase();
                if QUIT_COMMANDS.contains(&input) {
                    // TODO: como puedo matar a un thread desde si mismo? o mandar una señal a main para que lo mate
                    println!("quit");
                    break;
                }
                else if STAT_COMMANDS.contains(&input) {
                    let x = statistics_keyboard.to_owned().get_total_count();
                    println!("Total number of reservations: {}", x);
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
        keyboard_listener(statistics_keyboard);
    });


    // Print statistics
    // print!("Total count {} \n", statistics.get_total_count());
    // print!("Total sum time {} \n", statistics.get_sum_time());
    // for (key, value) in statistics.get_destinations().iter() {
    //     println!("{} -> Total count: {}", key, value);
    // }
    // for (k, v) in statistics.get_top_destinations(3) {
    //     println!("{}. Total count: {}", k, v);
    // }
    // print!("Avg time: {:.2} \n", statistics.get_avg_time());

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                airlines: airlines::from_file(&filename_airline),
                statistics: statistics_webserver.to_owned(),
            })
            .service(reservation)
    })
    .bind(("127.0.0.1", 8080))?
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

    let flight: FlightReservation = FlightReservation {
        origin: req.origin.clone(),
        destination: req.destination.clone(),
        airline: req.airline.clone(),
        hotel: req.hotel.clone(),
    };

    let reservation = alglobo::reserve(flight, semaphore.unwrap().clone(), appstate.statistics.clone());

    reservation.join().unwrap();
    HttpResponse::Ok().finish()
}
