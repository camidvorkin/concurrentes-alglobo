//! Handle airlines config
extern crate actix;

use crate::flight::InfoFlight;
use crate::logger;
use crate::statsactor::Stat;
use actix::{Actor, Addr, Handler, SyncArbiter, SyncContext};
use common::utils::{get_retry_seconds, read_file};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

use std::thread;
use std::time::Duration;

/// WebServer actor airline
pub struct Airline {}

impl Actor for Airline {
    type Context = SyncContext<Self>;
}

impl Handler<InfoFlight> for Airline {
    type Result = ();

    /// Handle the message of InfoFlight and simulates to send it to the server.
    /// If the server is not available, the message is sent again after a random time. If the server is available,
    /// the message is sent to the StatsActor for statistics purpuses.
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Airline as Actor>::Context) -> Self::Result {
        logger::log(format!(
            "Starting Request to Airline {} for route: [{}]",
            msg.flight_reservation.airline,
            msg.flight_reservation.get_route()
        ));
        loop {
            let retry_seconds = get_retry_seconds();

            thread::sleep(Duration::from_secs(1));
            let retry = thread_rng().gen_bool(0.4);
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
                // Handle if Result is Err
                match msg.addr_statistics.try_send(Stat {
                    elapsed_time: msg.start_time.elapsed().as_millis(),
                    flight_reservation: msg.flight_reservation,
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        logger::log("Request FAILED".to_string());
                    }
                };
                break;
            }
            thread::sleep(Duration::from_secs(retry_seconds));
            // thread::sleep(Duration::from_secs(2));
        }
    }
}

pub type Airlines = HashMap<String, Addr<Airline>>;

/// Create a Airline Server for each available airline in filename.
/// Each Server allows to launch multiple instances of the specific Actor Airline according to the rate limit of the server.
/// Returns a HashMap with the name of the airline and the address of the server.
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
