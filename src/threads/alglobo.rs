//! Make the reservations
use crate::flight_reservation::FlightReservation;
use crate::statistics::Statistics;
use rand::Rng;
use std::env;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;

/// If the user doesn't set the ENVVAR `RETRY_SECONDS` we default to this value
const DEFAULT_RETRY_SECONDS: u64 = 5;

/// Simulated request to an hypothetical hotel web server
fn simulate_hotel() -> bool {
    thread::sleep(Duration::from_secs(1));
    true
}

/// Simulated request to an hypothetical airline web server
fn simulate_airline() -> bool {
    thread::sleep(Duration::from_secs(1));

    rand::thread_rng().gen_bool(0.8)
}

/// Function that makes the request to the hotel
fn send_to_hotel(
    flight_info: FlightReservation,
    barrier: Arc<Barrier>,
    logger_sender: Sender<String>,
) {
    if flight_info.hotel && simulate_hotel() {
        let s = format!("[{}] Hotel Reservation: OK", flight_info.to_string());
        logger_sender.send(s).unwrap();
    }
    barrier.wait();
}

/// Function that makes the request to the airline
///
/// If the request was declined by the airline, we retry it in N seconds (either a default value, or the ENVVAR `RETRY_SECONDS`)
fn send_to_airline(
    flight_info: FlightReservation,
    sem: Arc<Semaphore>,
    barrier: Arc<Barrier>,
    logger_sender: Sender<String>,
) {
    let retry_seconds = match env::var("RETRY_SECONDS") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_) => DEFAULT_RETRY_SECONDS,
    };

    loop {
        if simulate_airline() {
            let s = format!("[{}] Flight Reservation: OK", flight_info.to_string());
            logger_sender.send(s).unwrap();

            sem.release();
            barrier.wait();
            break;
        }
        let s = format!("[{}] Flight Reservation: RETRY", flight_info.to_string());
        logger_sender.send(s).unwrap();
        thread::sleep(Duration::from_secs(retry_seconds));
    }
}

/// We make a reservation by sending the request to the airline webserver and, if we are dealing with packages, to the hotel server
///
/// The result is the union of this two responses
pub fn reserve(
    flight_reservation: FlightReservation,
    rate_limit: Arc<Semaphore>,
    mut statistics: Statistics,
    logger_sender: Sender<String>,
) {
    let start_time = std::time::Instant::now();

    let flight_hotel = flight_reservation.clone();
    let flight_airline = flight_reservation.clone();
    let flight_path = flight_reservation.get_route();

    let barrier = Arc::new(Barrier::new(3));
    let barrier_hotel = barrier.clone();
    let barrier_airline = barrier.clone();

    let logger_sender_hotel = logger_sender.clone();
    let logger_sender_airline = logger_sender;

    rate_limit.acquire();

    // Send request to the airline and hotel (if requested) concurrently
    thread::Builder::new()
        .name("hotel".to_string())
        .spawn(move || send_to_hotel(flight_hotel, barrier_hotel, logger_sender_hotel))
        .expect("thread creation failed");

    thread::Builder::new()
        .name(flight_reservation.airline)
        .spawn(move || {
            send_to_airline(
                flight_airline,
                rate_limit,
                barrier_airline,
                logger_sender_airline,
            )
        })
        .expect("thread creation failed");

    barrier.wait();
    statistics.add_flight_reservation(start_time, flight_path);
}