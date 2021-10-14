//! Make the reservations
use std::env;
use std::thread;
use rand::Rng;
use std::time::Duration;
use crate::flight_reservation::FlightReservation;
use crate::statistics::Statistics;
use std_semaphore::Semaphore;
use std::sync::{Arc, Barrier};

/// If the user doesn't set the ENVVAR `RETRY_SECONDS` we default to this value
const DEFAULT_RETRY_SECONDS: i32 = 5;

/// Simulated request to an hypothetical hotel web server
fn simulate_hotel() -> bool {
    thread::sleep(Duration::from_secs(1));
    true
}

/// Simulated request to an hypothetical airline web server
fn simulate_airline() -> bool {
    thread::sleep(Duration::from_secs(1));
    let rng = rand::thread_rng().gen_bool(0.8);
    rng
}

/// Function that makes the request to the hotel
fn send_to_hotel(flight_info: FlightReservation, barrier: Arc<Barrier>) -> () {
    if flight_info.hotel {
        if simulate_hotel() {
            print!("[{}] Hotel Reservation: OK \n", flight_info.to_string());
        }
    }
    barrier.wait();
}

/// Function that makes the request to the airline
///
/// If the request was declined by the airline, we retry it in N seconds (either a default value, or the ENVVAR `RETRY_SECONDS`)
fn send_to_airline(flight_info: FlightReservation, sem: Arc<Semaphore>, barrier: Arc<Barrier>) -> () {
    let retry_seconds = match env::var("RETRY_SECONDS") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_) => DEFAULT_RETRY_SECONDS,
    };

    loop {
        if simulate_airline() {
            print!("[{}] Flight Reservation: OK \n", flight_info.to_string());
            sem.release();
            barrier.wait();
            break;
        }
        print!("[{}] Flight Reservation: RETRY \n", flight_info.to_string());
        thread::sleep(Duration::from_secs(retry_seconds));
    }
}

/// After the requests are done, we add the flight to our statistics
fn end_transaction(mut statistics: Statistics, barrier: Arc<Barrier>, start_time: std::time::Instant, flight_path: String) -> () {
    barrier.wait();
    statistics.add_flight_reservation(start_time, flight_path);
}

/// We make a reservation by sending the request to the airline webserver and, if we are dealing with packages, to the hotel server
///
/// The result is the union of this two responses
pub fn reserve(flight_reservation: FlightReservation, rate_limit:Arc<Semaphore>, statistics: Statistics) -> thread::JoinHandle<()> {
    let start_time = std::time::Instant::now();

    let flight_hotel = flight_reservation.clone();
    let flight_airline = flight_reservation.clone();
    let flight_path = flight_reservation.get_route();

    let barrier = Arc::new(Barrier::new(3));
    let barrier_hotel = barrier.clone();
    let barrier_airline = barrier.clone();

    rate_limit.acquire();

    // Send request to the airline and hotel (if requested) concurrently
    thread::spawn( move || {
        send_to_hotel(flight_hotel, barrier_hotel)
    });
    thread::spawn( move || {
        send_to_airline(flight_airline, rate_limit, barrier_airline)
    });

    // Wait for transaction to be over to add statistics
    thread::spawn( move || {
        end_transaction(statistics, barrier, start_time, flight_path);
        println!("")
    })
}
