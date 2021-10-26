//! Hotel request actor
extern crate actix;

use crate::info_flight::InfoFlight;
use crate::stats_actor::{Stat, StatsActor, FinishMessage};
use actix::{Actor, Addr, Handler, Context, ResponseActFuture, ActorFutureExt, WrapFuture};
use common::logger;
use common::logger::LogLevel;
// use common::simulate_requests::simulate_hotel;
use std::time::Duration;
use actix::clock::sleep;

pub struct Hotel {
    /// Ref to the stats actor
    pub addr_statistics: Addr<StatsActor>,
}

impl Actor for Hotel {
    type Context = Context<Self>;
}

impl Handler<InfoFlight> for Hotel {
    type Result =  ResponseActFuture<Self, ()>;

    /// Handle the message of InfoFlight and simulates to send it to the Hotel server if the request includes the whole package experience.
    ///
    /// The server is always available so the request is always successful.
    fn handle(&mut self, msg: InfoFlight, _ctx: &mut Self::Context) -> Self::Result {
        logger::log(
            format!("{} | HOTEL   | Request started", msg.flight_reservation),
            LogLevel::INFO,
        );

        Box::pin(sleep(Duration::from_secs(1))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                logger::log(
                    format!("{} | HOTEL   | Request accepted", msg.flight_reservation),
                    LogLevel::INFO,
                );
                let _ = me.addr_statistics.try_send(Stat {
                    elapsed_time: msg.start_time.elapsed().as_millis(),
                    flight_reservation: msg.flight_reservation,
                });
            }))
    }
}

impl Handler<FinishMessage> for Hotel {
    type Result = Result<(i64, i64, f64), ()>;

    /// Shutdown handler that returns the total stats
    fn handle(&mut self, _msg: FinishMessage, _ctx: &mut Self::Context) -> Self::Result {
        Ok((
            0,
            0,
            0.0,
        ))
    }
}