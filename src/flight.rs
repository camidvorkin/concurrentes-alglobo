use crate::flight_reservation::FlightReservation;
use crate::statsactor::StatsActor;
use actix::{Addr, Message};

/// Message made to a Actor Airline to start the request of flight reservation to the server.
/// The message contains the address of the StatsActor for statistics purpuses and the information of the flight reservation.
pub struct InfoFlight {
    pub flight_reservation: FlightReservation,
    pub addr_statistics: Addr<StatsActor>,
    pub start_time: std::time::Instant,
}

impl Clone for InfoFlight {
    fn clone(&self) -> Self {
        InfoFlight {
            flight_reservation: self.flight_reservation.clone(),
            addr_statistics: self.addr_statistics.clone(),
            start_time: self.start_time.clone(),
        }
    }
}

impl Message for InfoFlight {
    type Result = ();
}
