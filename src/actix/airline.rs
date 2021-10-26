//! Airline request actor
extern crate actix;

use std::time::Duration;

use crate::airline_manager::FinishRequest;
use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use actix::clock::sleep;
use actix::{
    Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, ResponseActFuture, WrapFuture,
};
use common::logger::{self, LogLevel};
use common::utils::{get_retry_seconds, toin_coss};
use common::{MAX_TIME, MIN_TIME};

use rand::{thread_rng, Rng};

/// Airline Actor Struct
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
    /// If the simulation fails, we have to send the flight again after N seconds. We do this by sending the message back to itself as a retry, and then waiting longer on the next run
    ///
    /// If the simulation succeeds, we inform the AirlineManager and the StatsActor
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        if !msg.is_retry {
            // If we are retrying a flight, the logger has already reported it
            logger::log(
                format!("{} | AIRLINE | Request started", msg.flight_reservation),
                LogLevel::INFO,
            );
        }
        let mut sleep_seconds = thread_rng().gen_range(MIN_TIME, MAX_TIME);
        let retry_seconds = get_retry_seconds();
        if msg.is_retry {
            sleep_seconds += retry_seconds;
        }
        Box::pin(
            sleep(Duration::from_secs(sleep_seconds))
                .into_actor(self)
                .map(move |_result, me, ctx| match toin_coss() {
                    Err(_) => {
                        logger::log(
                            format!(
                                "{} | AIRLINE | Request rejected ; Retry in {} seconds",
                                msg.flight_reservation, retry_seconds
                            ),
                            LogLevel::INFO,
                        );

                        let mut retry_flight = msg;
                        retry_flight.is_retry = true;
                        let _ = ctx.notify(retry_flight);
                    }
                    Ok(()) => {
                        logger::log(
                            format!("{} | AIRLINE | Request accepted", msg.flight_reservation,),
                            LogLevel::INFO,
                        );
                        me.addr_statistics.do_send(Stat {
                            elapsed_time: msg.start_time.elapsed().as_millis(),
                            flight_reservation: msg.clone().flight_reservation,
                        });

                        msg.addr_manager.do_send(FinishRequest {
                            info_flight: msg.clone(),
                        });
                    }
                }),
        )
    }
}
