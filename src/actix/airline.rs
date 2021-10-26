//! Airline request actor
extern crate actix;

use std::thread;
use std::time::Duration;

use actix::{Actor, ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture, AsyncContext, Addr};
use actix::clock::sleep;
use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use common::logger::{self, LogLevel};
// use common::simulate_requests::simulate_airline;
use common::utils::get_retry_seconds;
use rand::{Rng, thread_rng};
use crate::airline_manager::FinishRequest;

pub struct Airline {
    /// Ref to the stats actor
    pub addr_statistics: Addr<StatsActor>
}

impl Actor for Airline {
    type Context = Context<Self>;
}


impl Handler<InfoFlight> for Airline {
    type Result = ResponseActFuture<Self, ()>;

    /// Handle the message of InfoFlight and simulates to send it to the server.
    ///
    /// If the server is not available, the message is retried after N seconds
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        logger::log(
            format!("{} | AIRLINE | Request started", msg.flight_reservation),
            LogLevel::INFO,
        );
        let retry_seconds = get_retry_seconds();
        Box::pin(sleep(Duration::from_secs(thread_rng().gen_range(500, 1500)))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                match rand::thread_rng().gen_bool(0.5) {
                    false => { 
                        logger::log(
                            format!("{} | AIRLINE | Request rejected ; Retry in {} seconds",
                            msg.flight_reservation, retry_seconds),
                            LogLevel::INFO,
                        );
                    
                        thread::sleep(Duration::from_secs(retry_seconds));
                        let _ = ctx.address().try_send(msg); 
                    }
                    true => {
                        logger::log(
                            format!("{} | AIRLINE | Request accepted", msg.flight_reservation,),
                            LogLevel::INFO,
                        );
                        let a = msg.clone();
                        let _ = me.addr_statistics.try_send(Stat {
                            elapsed_time: msg.start_time.elapsed().as_millis(),
                            flight_reservation: msg.flight_reservation,
                        });
                        
                        let __ = a.addr_manager.try_send(FinishRequest {
                            info_flight: a.clone(),
                        });
                    }
                }
            }))
    }
}

