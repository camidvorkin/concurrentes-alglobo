//! Make the reservations
use crate::statistics::Statistics;
use common::flight_reservation::FlightReservation;
use common::simulate_requests::{simulate_airline, simulate_hotel};
use common::utils::get_retry_seconds;

use std::sync::mpsc::Sender;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;

/// Function that makes the request to the hotel
fn send_to_hotel(
    flight_info: FlightReservation,
    barrier: Arc<Barrier>,
    logger_sender: Sender<String>,
) {
    let retry_seconds = get_retry_seconds();

    while let Err(_) = simulate_hotel() {
        let s = format!("[{}] Hotel Reservation: RETRY", flight_info.to_string());
        logger_sender
            .send(s)
            .expect("Logger mpsc not receving messages");
        ();
        thread::sleep(Duration::from_secs(retry_seconds));
    }
    let s = format!("[{}] Hotel Reservation: OK", flight_info.to_string());
    logger_sender
        .send(s)
        .expect("Logger mpsc not receving messages");
    ();
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
    let retry_seconds = get_retry_seconds();

    while let Err(_) = simulate_airline() {
        let s = format!("[{}] Flight Reservation: RETRY", flight_info.to_string());
        logger_sender
            .send(s)
            .expect("Logger mpsc not receving messages");
        ();
        thread::sleep(Duration::from_secs(retry_seconds));
    }
    let s = format!("[{}] Flight Reservation: OK", flight_info.to_string());
    logger_sender
        .send(s)
        .expect("Logger mpsc not receving messages");
    ();
    sem.release();
    barrier.wait();
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
    let logger_sender_airline = logger_sender.clone();

    rate_limit.acquire();

    // Send request to the airline and hotel (if requested) concurrently

    if flight_reservation.hotel {
        thread::Builder::new()
            .name("hotel".to_string())
            .spawn(move || send_to_hotel(flight_hotel, barrier_hotel, logger_sender_hotel))
            .expect("thread creation failed");
    }

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
