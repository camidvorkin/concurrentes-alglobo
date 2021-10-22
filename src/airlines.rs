//! Handle airlines config
extern crate actix;

use crate::flight_reservation::FlightReservation;
use crate::statsactor::{StatsActor, XXX};
use crate::utils::read_file;
use actix::Message;
use actix::{Actor, Addr, Handler, SyncArbiter, SyncContext};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

const DEFAULT_RETRY_SECONDS: u64 = 5;

/// WebServer actor airline
pub struct Airline {}

impl Actor for Airline {
    type Context = SyncContext<Self>;
}

pub struct InfoFlight {
    pub flight_reservation: FlightReservation,
    pub addr_statistics: Addr<StatsActor>,
}

impl Message for InfoFlight {
    type Result = ();
}

impl Handler<InfoFlight> for Airline {
    type Result = ();

    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Airline as Actor>::Context) -> Self::Result {
        let _retry_seconds = match env::var("RETRY_SECONDS") {
            Ok(val) => val.parse::<u64>().unwrap(),
            Err(_) => DEFAULT_RETRY_SECONDS,
        };
        loop {
            thread::sleep(Duration::from_secs(1));
            if !thread_rng().gen_bool(0.8) {
                let s = format!(
                    "[{}] Flight Reservation: RETRY",
                    msg.flight_reservation.get_route().to_string()
                );
                println!("{}", s);
                thread::sleep(Duration::from_secs(2));
            } else {
                let s = format!(
                    "[{}] Flight Reservation: SUCESSFUL",
                    msg.flight_reservation.get_route().to_string()
                );
                msg.addr_statistics.do_send(XXX {
                    s: "aaaaa".to_string(),
                });
                println!("{}", s);
                break;
            }
        }
    }
}

pub type Airlines = HashMap<String, Addr<Airline>>;

pub fn from_file(filename: &str) -> Airlines {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Addr<Airline>>::new();
    for airline in airlines {
        let airline_actor = SyncArbiter::start(airline[1].parse::<usize>().unwrap(), || Airline {});
        airline_map.insert(airline[0].to_string(), airline_actor);
    }
    airline_map
}
