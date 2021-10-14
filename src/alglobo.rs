
use std::thread;
use rand::Rng;
use std::time::Duration;
use crate::flight_reservation::FlightReservation;
use crate::statistics::Statistics;
use std_semaphore::Semaphore;
use std::sync::{Arc, Barrier};

fn _send_to_hotel() -> () {
    // thread::sleep(Duration::from_millis(thread_rng().gen_range(0, 10000)));
    thread::sleep(Duration::from_secs(1));
    print!("Hotel reservation successful \n");
}

// Request to hotels
fn send_to_hotel(has_hotel: bool, barrier: Arc<Barrier>) -> () {
    if has_hotel {
        _send_to_hotel();
    }
    barrier.wait();
}

// Request to airline. Returns true if request was accepted, false otherwise
fn _send_to_airline() -> bool {
    // thread::sleep(Duration::from_millis(thread_rng().gen_range(1000, 10000)));
    thread::sleep(Duration::from_secs(1));
    let rng = rand::thread_rng().gen_bool(0.8);
    rng
}

fn send_to_airline(flight_info: FlightReservation, sem: Arc<Semaphore>, barrier: Arc<Barrier>) -> () {
    loop {
        if _send_to_airline() {
            print!("Flight reservation successful for {}. For flight: {} \n", flight_info.airline, flight_info.get_flight_code());
            sem.release();
            barrier.wait();
            break;
        }
        print!("Flight reservation failed for {}. For flight: {} \n", flight_info.airline, flight_info.get_flight_code());
        // thread::sleep(Duration::from_millis(thread_rng().gen_range(0, 5000)));
        thread::sleep(Duration::from_millis(5000));
    }
}

fn end_transaction(mut statistics: Statistics, barrier: Arc<Barrier>, start_time: std::time::Instant, flight_path: String) -> () {
    barrier.wait();
    statistics.add_flight_reservation(start_time, flight_path);
}

// Request flight
pub fn reserve(flight_reservation: FlightReservation, rate_limit:Arc<Semaphore>, statistics: Statistics) -> thread::JoinHandle<()> {
    let start_time = std::time::Instant::now();

    let flight = flight_reservation.clone();
    let flight_path = flight_reservation.get_path();

    let barrier_c1 = Arc::new(Barrier::new(3));
    let barrier_c2 = barrier_c1.clone();
    let barrier_c3 = barrier_c1.clone();

    let sem = rate_limit.clone();
    sem.acquire();

    // Send request to the airline and hotel(if was requested) simultaneously
    let _handle1 = thread::spawn( move || {
        send_to_hotel(flight_reservation.hotel, barrier_c1)
    });
    let _handle2 = thread::spawn( move || {
        send_to_airline(flight, sem, barrier_c2)
    });

    // Wait for transaction to be over to add statistics
    thread::spawn( move || {
        end_transaction(statistics, barrier_c3, start_time, flight_path)
    })
}
