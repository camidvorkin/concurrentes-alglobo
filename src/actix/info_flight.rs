//! Flight information message to be sent accross Hotel and Airline actors
use actix::Message;
use common::flight_reservation::FlightReservation;

/// Message made to a Actor Airline or Hotel to start the request of flight reservation to the server.
///
/// The message contains the address of the StatsActor for statistics purpuses and the information of the flight reservation.
#[derive(Message)]
#[rtype(result = "()")]
pub struct InfoFlight {
    /// The flight itself
    pub flight_reservation: FlightReservation,
    /// When the flight started to being processed by the program
    pub start_time: std::time::Instant,
}

impl Clone for InfoFlight {
    fn clone(&self) -> Self {
        InfoFlight {
            flight_reservation: self.flight_reservation.clone(),
            start_time: self.start_time,
        }
    }
}
