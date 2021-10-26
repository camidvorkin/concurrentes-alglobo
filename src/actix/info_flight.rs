//! Flight information message to be sent accross Hotel and Airline actors
use crate::airline_manager::AirlineManager;
use actix::Addr;
use actix::Message;
use common::flight_reservation::FlightReservation;

/// Message sent to an AirlineManager or a Hotel to start the request of flight reservation to the server
#[derive(Message)]
#[rtype(result = "()")]
pub struct InfoFlight {
    /// The flight itself
    pub flight_reservation: FlightReservation,
    /// When the flight started to being processed by the program
    pub start_time: std::time::Instant,
    /// A reference to the airline manager, to send a FinishRequest message
    pub addr_manager: Addr<AirlineManager>,
    /// Indicates if the flight is a new one, or is being retried
    pub is_retry: bool,
}

impl Clone for InfoFlight {
    fn clone(&self) -> Self {
        InfoFlight {
            flight_reservation: self.flight_reservation.clone(),
            start_time: self.start_time,
            addr_manager: self.addr_manager.clone(),
            is_retry: self.is_retry,
        }
    }
}
