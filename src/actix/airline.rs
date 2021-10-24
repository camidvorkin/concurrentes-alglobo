//! Handle airlines config
extern crate actix;

use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use actix::{Actor, Addr, Handler, SyncArbiter, SyncContext};
use common::logger::{self, LogLevel};
use common::simulate_requests::simulate_airline;
use common::utils::{get_retry_seconds, read_file};

use std::collections::HashMap;

use std::thread;
use std::time::Duration;

/// WebServer actor airline
pub struct Airline {
    pub addr_statistics: Addr<StatsActor>,
}

impl Actor for Airline {
    type Context = SyncContext<Self>;
}

impl Handler<InfoFlight> for Airline {
    type Result = ();

    /// Handle the message of InfoFlight and simulates to send it to the server.
    /// If the server is not available, the message is sent again after a random time. If the server is available,
    /// the message is sent to the StatsActor for statistics purpuses.
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        logger::log(
            format!("{} | AIRLINE | Request started", msg.flight_reservation),
            LogLevel::INFO,
        );
        let retry_seconds = get_retry_seconds();

        while simulate_airline().is_err() {
            logger::log(
                format!(
                    "{} | AIRLINE | Request rejected ; Retry in {} seconds",
                    msg.flight_reservation, retry_seconds
                ),
                LogLevel::INFO,
            );
            thread::sleep(Duration::from_secs(retry_seconds));
        }

        logger::log(
            format!("{} | AIRLINE | Request accepted", msg.flight_reservation,),
            LogLevel::INFO,
        );

        let _ = self.addr_statistics.try_send(Stat {
            elapsed_time: msg.start_time.elapsed().as_millis(),
            flight_reservation: msg.flight_reservation,
        });
    }
}

/// Create a Airline Server for each available airline in filename.
/// Each Server allows to launch multiple instances of the specific Actor Airline according to the rate limit of the server.
/// Returns a HashMap with the name of the airline and the address of the server.
pub fn from_file(
    filename: &str,
    addr_statistics: Addr<StatsActor>,
) -> HashMap<String, Addr<Airline>> {
    let airlines = read_file(filename).expect("Couldn't read airline file");
    let mut airline_map = HashMap::<String, Addr<Airline>>::new();
    for airline in airlines {
        let addr_statistics_airline = addr_statistics.clone();
        let airline_actor = SyncArbiter::start(
            airline[1].parse::<usize>().expect("Couldn't parse airline"),
            move || Airline {
                addr_statistics: addr_statistics_airline.to_owned(),
            },
        );
        airline_map.insert(airline[0].to_string(), airline_actor);
    }
    airline_map
}
