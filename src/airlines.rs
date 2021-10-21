//! Handle airlines config
extern crate actix;

use std::time::Duration;
use actix::{Actor, Handler, Message, SyncArbiter, SyncContext, Addr};
use actix::clock::sleep;
use std::env;
use crate::utils::read_file;
use std::collections::HashMap;
use crate::flight_reservation::FlightReservation;
use rand::{thread_rng, Rng};

/// If the user doesn't set the ENVVAR `RETRY_SECONDS` we default to this value
const DEFAULT_RETRY_SECONDS: u64 = 5;

/// Message to start the actor with the flight information
pub struct InfoFlight {
    pub flight_reservation: FlightReservation
}

impl Message for InfoFlight {
    type Result = i32;
}

/// WebServer actor airline
pub struct Airline { 
}

impl Actor for Airline {
    type Context = SyncContext<Self>; 
}

/// Airline handle the request for the flight information.
/// Not all the request are accepted at once. If the request is not accepted, the actor will retry `RETRY_SECONDS` seconds later.
impl Handler<InfoFlight> for Airline {
    type Result = i32;

    fn handle(&mut self, msg: InfoFlight, _ctx: &mut <Airline as Actor>::Context) -> Self::Result {
        
        let retry_seconds = match env::var("RETRY_SECONDS") {
            Ok(val) => val.parse::<u64>().unwrap(),
            Err(_) => DEFAULT_RETRY_SECONDS,
        };
    
        loop {
            let s = sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)));
            if !thread_rng().gen_bool(0.8) {
                // Todo: send logger message
                let s = format!("[{}] Flight Reservation: RETRY", msg.flight_reservation.get_route().to_string());
                print!("{}", s);
                sleep(Duration::from_secs(retry_seconds));
            }
            let s = format!("[{}] Flight Reservation: SUCESSFUL", msg.flight_reservation.get_route().to_string());
            print!("{}", s);
        }
    }
        
} 

/// Keep track of how many actors can each airline handle
pub type Airlines = HashMap<String, Addr<Airline>>;

/// Read from a CSV file with airlines and their max number of concurrent requests as columns and convert it into our Airlines type
pub fn from_file(filename: &str) -> Airlines {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Addr<Airline>>::new();
    for airline in airlines {
        let airline_actor = SyncArbiter::start(airline[1].parse::<usize>().unwrap(), || Airline {});
        airline_map.insert(airline[0].to_string(), airline_actor);
    }
    airline_map
}
