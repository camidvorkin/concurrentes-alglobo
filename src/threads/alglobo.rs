//! Make the reservations
use crate::statistics::Statistics;
use common::flight_reservation::FlightReservation;
use common::logger::{LogLevel, LoggerMsg};
use common::simulate_requests::{simulate_airline, simulate_hotel};
use common::utils::get_retry_seconds;

use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;

/// Function that makes the request to the hotel
fn send_to_hotel(
    flight_info: FlightReservation,
    pair: Arc<(Mutex<i16>, Condvar)>,
    logger_sender: Sender<LoggerMsg>,
) {
    logger_sender
        .send((
            format!("{} | HOTEL   | Request started", flight_info),
            LogLevel::INFO,
        ))
        .expect("Logger mpsc not receving messages");

    simulate_hotel();
    logger_sender
        .send((
            format!("{} | HOTEL   | Request accepted", flight_info),
            LogLevel::INFO,
        ))
        .expect("Logger mpsc not receving messages");

    let (lock, cvar) = &*pair;
    cvar.notify_all();
    let mut pending = lock.lock().expect("Unable to lock");
    *pending += 1;
    cvar.notify_all();
}

/// Function that makes the request to the airline
///
/// If the request was declined by the airline, we retry it in N seconds (either a default value, or the ENVVAR `RETRY_SECONDS`)
fn send_to_airline(
    flight_info: FlightReservation,
    sem: Arc<Semaphore>,
    pair: Arc<(Mutex<i16>, Condvar)>,
    logger_sender: Sender<LoggerMsg>,
) {
    logger_sender
        .send((
            format!("{} | AIRLINE | Request started", flight_info),
            LogLevel::INFO,
        ))
        .expect("Logger mpsc not receving messages");

    let retry_seconds = get_retry_seconds();

    while simulate_airline().is_err() {
        logger_sender
            .send((
                format!(
                    "{} | AIRLINE | Request rejected ; Retry in {} seconds",
                    flight_info, retry_seconds
                ),
                LogLevel::INFO,
            ))
            .expect("Logger mpsc not receving messages");

        thread::sleep(Duration::from_secs(retry_seconds));
    }
    logger_sender
        .send((
            format!("{} | AIRLINE | Request accepted", flight_info),
            LogLevel::INFO,
        ))
        .expect("Logger mpsc not receving messages");

    sem.release();

    let (lock, cvar) = &*pair;
    cvar.notify_all();
    let mut pending = lock.lock().expect("Unable to lock");
    *pending += 1;
    cvar.notify_all();
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

    let complete_request = Arc::new((Mutex::new(0), Condvar::new()));
    let completed_with = if flight_reservation.hotel { 2 } else { 1 };
    let complete_request_hotel = complete_request.clone();
    let complete_request_airline = complete_request.clone();

    let logger_sender_hotel = logger_sender.clone();
    let logger_sender_airline = logger_sender.clone();

    rate_limit.acquire();

    // Send request to the airline and hotel (if requested) concurrently
    if flight_reservation.hotel {
        thread::Builder::new()
            .name("hotel".to_string())
            .spawn(move || send_to_hotel(flight_hotel, complete_request_hotel, logger_sender_hotel))
            .expect("thread creation failed");
    }

    thread::Builder::new()
        .name(flight_reservation.clone().airline)
        .spawn(move || {
            send_to_airline(
                flight_airline,
                rate_limit,
                complete_request_airline,
                logger_sender_airline,
            )
        })
        .expect("thread creation failed");

    let (lock, cvar) = &*complete_request;

    let mut completed = lock.lock().expect("Unable to lock");
    while *completed != completed_with {
        completed = cvar.wait(completed).expect("Error on wait condvar");
    }
    statistics.add_flight_reservation(start_time, flight_path);
    logger_sender
        .send((
            format!(
                "{} | FINISH  | The flight was processed in {} millis. ",
                flight_reservation,
                start_time.elapsed().as_millis()
            ),
            LogLevel::INFO,
        ))
        .expect("Logger mpsc not receving messages");
}
