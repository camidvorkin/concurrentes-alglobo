
mod alglobo;
mod statistics;
use crate::statistics::Statistics;
use common::flight_reservation::FlightReservation;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Barrier};
use std_semaphore::Semaphore;


#[cfg(test)]
mod tests {
    use super::*;

    const NUM_SLEEP : u32 = 10;

    #[test]
    fn test1() {
        let (logger_sender, logger_receiver) = mpsc::channel();
        let statistics = Statistics::new();
        let airline = Arc::new(Semaphore::new(1));
        let flight_reservation = FlightReservation {
            id: 1,
            origin: String::from("origin"),
            destination: String::from("destination"),
            airline: String::from("airline"),
            hotel: false,
        };

        alglobo::reserve(
            flight_reservation,
            airline.to_owned(),
            statistics.to_owned(),
            logger_sender.to_owned(),
        );
        assert_eq!(statistics.get_total_count(), 1);
    }

}