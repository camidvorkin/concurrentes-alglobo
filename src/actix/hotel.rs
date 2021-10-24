//! Handle airlines config
extern crate actix;

use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use actix::{Actor, Addr, Handler, SyncContext};
use common::logger;
use common::logger::LogLevel;
use common::simulate_requests::simulate_hotel;
use common::utils::get_retry_seconds;

pub struct Hotel {
    pub addr_statistics: Addr<StatsActor>,
}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<InfoFlight> for Hotel {
    type Result = ();

    /// Handle the message of InfoFlight and simulates to send it to the Hotel server if the request includes the whole package experience.
    /// The server is always available so the request is always successful.
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        simulate_hotel();
        logger::log(
            format!("{} | HOTEL REQUEST   | OK", msg.flight_reservation),
            LogLevel::INFO,
        );

        let _ = self.addr_statistics.try_send(Stat {
            elapsed_time: msg.start_time.elapsed().as_millis(),
            flight_reservation: msg.flight_reservation,
        });
    }
}
