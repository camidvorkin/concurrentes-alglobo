//! Handle airlines config
extern crate actix;

use crate::flight_reservation::FlightReservation;
use crate::logger;
use crate::statsactor::{Stat, StatsActor};
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
    pub start_time: std::time::Instant,
}

impl Message for InfoFlight {
    type Result = ();
}

impl Handler<InfoFlight> for Airline {
    type Result = ();

    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Airline as Actor>::Context) -> Self::Result {
        // logger::log(format!("Starting Request to Airline {} for route: [{}]", msg.flight_reservation.airline, msg.flight_reservation.get_route()));
        let _retry_seconds = match env::var("RETRY_SECONDS") {
            Ok(val) => val.parse::<u64>().unwrap(),
            Err(_) => DEFAULT_RETRY_SECONDS,
        };
        loop {
            // thread::sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
            thread::sleep(Duration::from_secs(1));
            let retry = false; //thread_rng().gen_bool(0.4);
            logger::log(
                format!(
                    "Request to {} for route [{}]: {}",
                    msg.flight_reservation.airline,
                    msg.flight_reservation.id,
                    if retry { "RETRY" } else { "SUCCESFUL" }
                )
                .to_string(),
            );

            if !retry {
                msg.addr_statistics.try_send(Stat {
                    elapsed_time: msg.start_time.elapsed().as_millis(),
                    flight_reservation: msg.flight_reservation,
                });
                break;
            }
            // thread::sleep(Duration::from_millis(thread_rng().gen_range(1000, 2000)));
            thread::sleep(Duration::from_secs(2));
        }
    }
}

pub type Airlines = HashMap<String, Addr<Airline>>;

pub fn from_file(filename: &str) -> Airlines {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Addr<Airline>>::new();
    for airline in airlines {
        logger::log(
            format!(
                "Creating Airline Server for {} with rate limite {}",
                airline[0].to_string(),
                airline[1].to_string()
            )
            .to_string(),
        );
        let airline_actor = SyncArbiter::start(airline[1].parse::<usize>().unwrap(), || Airline {});
        airline_map.insert(airline[0].to_string(), airline_actor);
    }
    airline_map
}
