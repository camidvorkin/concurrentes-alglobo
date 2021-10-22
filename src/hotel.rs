//! Handle airlines config
extern crate actix;

use crate::airlines::InfoFlight;

use crate::statsactor::{StatsActor, XXX};
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
        thread::sleep(Duration::from_secs(1));
        println!(
            "[{}] Hotel reservation: SUCESSFUL",
            msg.flight_reservation.get_route().to_string()
        );
        msg.addr_statistics.do_send(XXX {
            s: "hoteeeel".to_string(),
        });
    }
}

pub fn get_hotel_address(rate_limite: usize) -> Addr<Hotel> {
    SyncArbiter::start(rate_limite, || Hotel {})
}
