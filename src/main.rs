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

const AIRLINES_FILE: &str = "src/configs/airlines.txt";

/// TODO
struct AppState {
    /// TODO
    airlines: Airlines,
    /// TODO
    statistics: Statistics,
}

/// TODO
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // The Airlines config file is either a CLA or hardcoded in our configs directory
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => AIRLINES_FILE.to_string(),
    };

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
                statistics: Statistics::new(),
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
    // No se encuentra algun aeropuerto -> not acceptable
    // Mal formato -> 400
    // Cualquier otra cosa -> Success! y manejamos los errores desde el json de retorno


    let flight: FlightReservation = FlightReservation {
        origin: req.origin.clone(),
        destination: req.destination.clone(),
        airline: req.airline.clone(),
        hotel: req.hotel.clone(),
    };

    let reservation = alglobo::reserve(flight, appstate.airlines.get(&req.airline).unwrap().clone(), appstate.statistics.clone());

    reservation.join().unwrap();

    HttpResponse::Ok().body("a")
}
