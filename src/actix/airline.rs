//! Airline request actor
extern crate actix;

use std::thread;
use std::time::Duration;

use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use actix::clock::sleep;
use actix::{
    Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, ResponseActFuture, WrapFuture,
};
use common::logger::{self, LogLevel};
// use common::simulate_requests::simulate_airline;
use crate::airline_manager::FinishRequest;
use common::utils::get_retry_seconds;
use rand::{thread_rng, Rng};

pub struct Airline {
    /// Ref to the stats actor
    pub addr_statistics: Addr<StatsActor>,
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
        if !msg.is_retry {
            logger::log(
                format!("{} | AIRLINE | Request started", msg.flight_reservation),
                LogLevel::INFO,
            );
        }
        let mut sleep_seconds = 1;
        let retry_seconds = 2;
        if msg.is_retry {
            sleep_seconds += retry_seconds;
        }
        Box::pin(
            sleep(Duration::from_secs(sleep_seconds))
                .into_actor(self)
                .map(
                    move |_result, me, ctx| match rand::thread_rng().gen_bool(0.5) {
                        false => {
                            logger::log(
                                format!(
                                    "{} | AIRLINE | Request rejected ; Retrying in {} seconds",
                                    msg.flight_reservation, retry_seconds
                                ),
                                LogLevel::INFO,
                            );

                            let mut retry_flight = msg.clone();
                            retry_flight.is_retry = true;
                            let _ = ctx.notify(retry_flight);
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
                    },
                ),
        )
    }
}
