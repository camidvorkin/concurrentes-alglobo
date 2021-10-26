//! Hotel request actor
extern crate actix;

use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor};
use actix::clock::sleep;
use actix::{Actor, ActorFutureExt, Addr, Context, Handler, ResponseActFuture, WrapFuture};
use common::logger::LogLevel;
use common::{logger, MAX_TIME, MIN_TIME};
use rand::{thread_rng, Rng};
use std::time::Duration;

/// Hotel Actor Struct
pub struct Hotel {
    /// Ref to the stats actor
    pub addr_statistics: Addr<StatsActor>,
}

impl Actor for Hotel {
    type Context = Context<Self>;
}

impl Handler<InfoFlight> for Hotel {
    type Result = ResponseActFuture<Self, ()>;

    /// Handle the message of InfoFlight and simulates to send it to the Hotel server if the request includes the whole package experience.
    ///
    /// The server is always available so the request is always successful, it's just a random sleep.
    ///
    /// After the request is done, it sends the Stat to the StatsActor
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        logger::log(
            format!("{} | HOTEL   | Request started", msg.flight_reservation),
            LogLevel::INFO,
        );

        Box::pin(
            sleep(Duration::from_secs(
                thread_rng().gen_range(MIN_TIME, MAX_TIME),
            ))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                logger::log(
                    format!("{} | HOTEL   | Request accepted", msg.flight_reservation),
                    LogLevel::INFO,
                );
                let _ = me.addr_statistics.do_send(Stat {
                    elapsed_time: msg.start_time.elapsed().as_millis(),
                    flight_reservation: msg.flight_reservation,
                });
            }),
        )
    }
}
