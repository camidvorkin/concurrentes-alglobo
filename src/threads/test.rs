use crate::alglobo::reserve;
use crate::statistics::Statistics;
use common::flight_reservation::FlightReservation;
use common::logger::LoggerMsg;
use common::MIN_TIME;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std_semaphore::Semaphore;

pub struct Test {
    pub flight_reservations: Vec<FlightReservation>,
    pub statistics: Statistics,
    pub airline: Arc<Semaphore>,
    pub logger_sender: Sender<LoggerMsg>,
    pub logger_receiver: Receiver<LoggerMsg>,
}

// Initialize struct Test
impl Test {
    pub fn new(n_flights: i64, hotel: Vec<bool>, rate_limit: i64) -> Test {
        let (logger_sender, logger_receiver) = mpsc::channel();
        let statistics = Statistics::new();
        let mut flights = vec![];
        for i in 0..n_flights {
            let flight = FlightReservation {
                id: 1,
                origin: "origin".to_string(),
                destination: "destination".to_string(),
                airline: "airline".to_string(),
                hotel: hotel[i as usize],
            };
            flights.push(flight);
        }
        let airline = Arc::new(Semaphore::new(rate_limit as isize));
        Test {
            flight_reservations: flights,
            statistics: statistics,
            airline: airline,
            logger_sender: logger_sender,
            logger_receiver: logger_receiver,
        }
    }
}

#[test]
fn test_simple_reservation_no_hotel() {
    let test1 = Test::new(1, vec![false], 1);

    reserve(
        test1
            .flight_reservations
            .get(0)
            .expect("Unknown flight index")
            .clone(),
        test1.airline.clone(),
        test1.statistics.clone(),
        test1.logger_sender,
    );
    assert_eq!(test1.statistics.get_total_count(), 1);
    assert!(test1.statistics.get_avg_time() >= (MIN_TIME * 1000) as f64);
    assert!(test1.statistics.get_sum_time() >= (MIN_TIME * 1000) as i64);
}

#[test]
fn test_two_not_simultaneous_request() {
    let test2 = Test::new(2, vec![false, false], 1);

    let mut result = vec![];

    for flight in test2.flight_reservations {
        let airline_clone = test2.airline.clone();
        let statistics_clone = test2.statistics.clone();
        let logger_sender_clone = test2.logger_sender.clone();

        result.push(thread::spawn(move || {
            reserve(flight, airline_clone, statistics_clone, logger_sender_clone)
        }));
    }

    for thread in result {
        thread.join().expect("Unable to join");
    }
    assert_eq!(test2.statistics.get_total_count(), 2);
    assert!(test2.statistics.get_avg_time() >= 1.5 * (MIN_TIME * 1000) as f64);
    assert!(test2.statistics.get_sum_time() >= 2 * (MIN_TIME * 1000) as i64);
}

#[test]
fn test_two_simultaneous_requests() {
    let test3 = Test::new(2, vec![false, false], 2);

    let mut result = vec![];

    for flight in test3.flight_reservations {
        let airline_clone = test3.airline.clone();
        let statistics_clone = test3.statistics.clone();
        let logger_sender_clone = test3.logger_sender.clone();

        result.push(thread::spawn(move || {
            reserve(flight, airline_clone, statistics_clone, logger_sender_clone)
        }));
    }

    for thread in result {
        thread.join().expect("Unable to join");
    }
    assert_eq!(test3.statistics.get_total_count(), 2);
    assert!(test3.statistics.get_avg_time() >= (MIN_TIME * 1000) as f64);
    assert!(test3.statistics.get_sum_time() >= 2 * (MIN_TIME * 1000) as i64);
}

#[test]
fn test_one_request_with_holding_hotel() {
    let test4 = Test::new(1, vec![true], 1);

    reserve(
        test4
            .flight_reservations
            .get(0)
            .expect("Unknown flight index")
            .clone(),
        test4.airline.clone(),
        test4.statistics.clone(),
        test4.logger_sender,
    );
    assert_eq!(test4.statistics.get_total_count(), 1);
    assert!(test4.statistics.get_avg_time() >= (MIN_TIME * 1000) as f64);
    assert!(test4.statistics.get_sum_time() >= (MIN_TIME * 1000) as i64);
}

#[test]
fn test_two_request_with_one_holding_hotel() {
    let test5 = Test::new(2, vec![false, false], 1);

    let mut result = vec![];

    for flight in test5.flight_reservations {
        let airline_clone = test5.airline.clone();
        let statistics_clone = test5.statistics.clone();
        let logger_sender_clone = test5.logger_sender.clone();

        result.push(thread::spawn(move || {
            reserve(flight, airline_clone, statistics_clone, logger_sender_clone)
        }));
    }

    for thread in result {
        thread.join().expect("Unable to join");
    }
    assert_eq!(test5.statistics.get_total_count(), 2);
    assert!(test5.statistics.get_avg_time() >= 1.5 * (MIN_TIME * 1000) as f64);
    assert!(test5.statistics.get_sum_time() >= 2 * (MIN_TIME * 1000) as i64);
}

#[test]
fn test_volume() {
    let n_flights: i64 = 1000;
    let rate_limit: i64 = 500;

    let avg = n_flights as f64 / rate_limit as f64;

    let mut hotel = vec![];
    for _ in 0..n_flights {
        hotel.push(rand::random::<bool>());
    }

    let test = Test::new(n_flights, hotel, rate_limit);

    let mut result = vec![];

    for flight in test.flight_reservations {
        let airline_clone = test.airline.clone();
        let statistics_clone = test.statistics.clone();
        let logger_sender_clone = test.logger_sender.clone();

        result.push(thread::spawn(move || {
            reserve(flight, airline_clone, statistics_clone, logger_sender_clone)
        }));
    }

    for thread in result {
        thread.join().expect("Unable to join");
    }
    assert_eq!(test.statistics.get_total_count(), n_flights);
    assert!(test.statistics.get_avg_time() >= avg * (MIN_TIME * 1000) as f64);
    assert!(test.statistics.get_sum_time() >= n_flights * (MIN_TIME * 1000) as i64);
}
