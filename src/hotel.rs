//! Handle airlines config
extern crate actix;

use crate::flight::InfoFlight;
use crate::logger;
use crate::statsactor::Stat;
use actix::{Actor, Addr, Handler, SyncArbiter, SyncContext};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

pub struct Hotel {}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<InfoFlight> for Hotel {
    type Result = ();

    /// Handle the message of InfoFlight and simulates to send it to the Hotel server if the request includes the whole package experience.
    /// The server is always available so the request is always successful.
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Hotel as Actor>::Context) -> Self::Result {
        if msg.flight_reservation.hotel {
            logger::log(format!(
                "Starting Request to Hotel: [{}]",
                msg.flight_reservation.get_route()
            ));
            thread::sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
            // thread::sleep(Duration::from_secs(2));
            match msg.addr_statistics.try_send(Stat {
                elapsed_time: msg.start_time.elapsed().as_millis(),
                flight_reservation: msg.flight_reservation.clone(),
            }) {
                Ok(_) => {}
                Err(_) => {
                    logger::log("Request FAILED".to_string());
                }
            };
            logger::log(
                format!(
                    "Request to Hotel for route [{}]: SUCCESFUL",
                    msg.flight_reservation.id
                )
                .to_string(),
            );
        }
    }
}

/// Creates one Hotel Server which allows `rate_limite` amount of requests at the same time.
/// Returns the `Addr` of the created servers.
pub fn get_hotel_address(rate_limite: usize) -> Addr<Hotel> {
    logger::log(
        format!(
            "Creating Hotel Server with their rate limite {}",
            rate_limite
        )
        .to_string(),
    );
    SyncArbiter::start(rate_limite, || Hotel {})
}
