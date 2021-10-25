//! Airline request actor
extern crate actix;

use std::{thread, io};
use std::time::{Duration, SystemTime};

use actix::{Actor, ActorFutureExt, Context, Handler, Message, ResponseActFuture, WrapFuture, AsyncContext, Addr};
use actix::clock::sleep;
use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use common::logger::{self, LogLevel};
use common::simulate_requests::simulate_airline;
use common::utils::{get_retry_seconds, read_file};
use std::pin::Pin;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Airline {
    /// Ref to the stats actor
    pub addr_statistics: Addr<StatsActor>,
    pub rate_limit: u64,
    pub n_requests: u64,
}

impl Actor for Airline {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewRequest {
    pub info_flight: InfoFlight,
}

impl Handler<InfoFlight> for Airline {
    type Result = ResponseActFuture<Self, ()>;

    /// Handle the message of InfoFlight and simulates to send it to the server.
    ///
    /// If the server is not available, the message is retried after N seconds
    fn handle(&mut self, msg: InfoFlight, ctx: &mut Self::Context) -> Self::Result {
        logger::log(
            format!("{} | AIRLINE | Request started", msg.flight_reservation),
            LogLevel::INFO,
        );
        let retry_seconds = get_retry_seconds();
        Box::pin(sleep(Duration::from_millis(thread_rng().gen_range(500, 1500)))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                match rand::thread_rng().gen_bool(0.8) {
                    false => { 
                        logger::log(
                            format!("{} | AIRLINE | Request rejected ; Retry in {} seconds",
                            msg.flight_reservation, retry_seconds),
                            LogLevel::INFO,
                        );
                    
                        sleep(Duration::from_secs(retry_seconds));
                        _ctx.address().send(msg); 
                    }
                    true => {
                        logger::log(
                            format!("{} | AIRLINE | Request accepted", msg.flight_reservation,),
                            LogLevel::INFO,
                        );
                        let _ = me.addr_statistics.try_send(Stat {
                            elapsed_time: msg.start_time.elapsed().as_millis(),
                            flight_reservation: msg.flight_reservation,
                        });
                        // self.n_requests -= 1;
                    }
                }
            }))
    }
}

/// Create an Airline actor for each available airline in file
///
/// Returns a HashMap with the name of the airline and the address of the actor.
pub fn from_file(
    filename: &str,
    addr_statistics: Addr<StatsActor>,
) -> HashMap<String, Addr<Airline>> {
    let airlines = read_file(filename).expect("Couldn't read airline file");
    let mut airline_map = HashMap::<String, Addr<Airline>>::new();
    for airline in airlines {
        let addr_statistics_airline = addr_statistics.clone();
        let airline_actor = Airline {
            addr_statistics: addr_statistics_airline,
            rate_limit: airline[1].parse::<u64>().unwrap(),
            n_requests: 0,
        }.start();
        airline_map.insert(airline[0].to_string(), airline_actor);
    }
    airline_map
}
