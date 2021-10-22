//! Handle airlines config
extern crate actix;

use crate::airlines::InfoFlight;
use crate::logger;
use actix::{Actor, Addr, Handler, Message, SyncArbiter, SyncContext};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

pub struct InfoPackage {
    pub route: String,
}

impl Message for InfoPackage {
    type Result = ();
}

pub struct Hotel {}

impl Actor for Hotel {
    type Context = SyncContext<Self>;
}

impl Handler<InfoFlight> for Hotel {
    type Result = ();

    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Hotel as Actor>::Context) -> Self::Result {
        if msg.flight_reservation.hotel {
            logger::log(format!("Starting Request to Hotel: [{}]", msg.flight_reservation.get_route()));
            // thread::sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
            thread::sleep(Duration::from_secs(1));
            logger::log(format!("Request to Hotel for route [{}]: SUCCESFUL", msg.flight_reservation.get_route()).to_string())
        }
    }
}

pub fn get_hotel_address(rate_limite: usize) -> Addr<Hotel> {
    logger::log(format!("Creating Hotel Server with their rate limite {}", rate_limite).to_string());
    SyncArbiter::start(rate_limite, || Hotel {})
}
