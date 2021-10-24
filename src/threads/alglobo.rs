//! Make the reservations
use crate::statistics::Statistics;
use common::flight_reservation::FlightReservation;
use common::logger::{LogLevel, LoggerMsg};
use common::simulate_requests::{simulate_airline, simulate_hotel};
use common::utils::get_retry_seconds;

use std::sync::mpsc::Sender;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;
use rand::{thread_rng, Rng};

/// Function that makes the request to the hotel
fn send_to_hotel(
    flight: FlightReservation,
    barrier: Arc<Barrier>,
    logger_sender: Sender<LoggerMsg>,
) {
    simulate_hotel();
    logger_sender
        .send((format!("{} | HOTEL REQUEST   | OK", flight), LogLevel::INFO))
        .expect("Logger mpsc not receving messages");
    barrier.wait();
}

/// Function that makes the request to the airline
///
/// If the request was declined by the airline, we retry it in N seconds (either a default value, or the ENVVAR `RETRY_SECONDS`)
fn send_to_airline(
    flight: FlightReservation,
    sem: Arc<Semaphore>,
    barrier: Arc<Barrier>,
    logger_sender: Sender<LoggerMsg>,
) {
    let retry_seconds = get_retry_seconds();

    while simulate_airline().is_err() {
        logger_sender
            .send((
                format!("{} | AIRLINE REQUEST | RETRY", flight),
                LogLevel::INFO,
            ))
            .expect("Logger mpsc not receving messages");

        thread::sleep(Duration::from_secs(retry_seconds));
    }
    logger_sender
        .send((format!("{} | AIRLINE REQUEST | OK", flight), LogLevel::INFO))
        .expect("Logger mpsc not receving messages");

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
    logger_sender: Sender<LoggerMsg>,
) {
    let start_time = std::time::Instant::now();

    let flight_hotel = flight_reservation.clone();
    let flight_airline = flight_reservation.clone();
    let flight_path = flight_reservation.get_route();

    let barrier = Arc::new(Barrier::new(if flight_reservation.hotel { 3 } else { 2 }));
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
        .name(flight_reservation.clone().airline)
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

    logger_sender
        .send((format!("{} | FINISH", flight_reservation), LogLevel::INFO))
        .expect("Logger mpsc not receving messages");
    statistics.add_flight_reservation(start_time, flight_path);
}
