use std::{collections::HashMap, env};
mod flight_reservation;
mod statistics;
mod utils;
mod alglobo;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, post};
use statistics::Statistics;
use utils::Airlines;

use crate::flight_reservation::FlightReservation;

struct AppState {
    airlines: Airlines,
    statistics: Statistics,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Process airlines
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => utils::AIRLINES_FILE.to_string(),
    };
    let airlines = utils::process_airlines(&filename_airline);

    let statistics = Statistics::new();

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
                airlines: airlines.to_owned(),
                statistics: statistics.to_owned()
            })
            .service(reservation)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

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
