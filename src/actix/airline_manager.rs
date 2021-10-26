//! AirlineManager actor
//!
//! This actor is the equivalent of a concurrency semaphore. It handles NewRequest messages, trying to process them (if there's room for it on the airline), and it handles FinishRequest messages, which serve to make room for new requests
use crate::airline::Airline;
use crate::info_flight::InfoFlight;
use crate::stats_actor::StatsActor;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use common::utils::read_file;
use std::collections::HashMap;

/// AirlineManager Actor Struct
pub struct AirlineManager {
    /// A HashMap of all the airlines, with a tuple referring to the rate limit and the number of current flights being processed
    pub requests_per_airline: HashMap<String, (u64, u64)>,
    // A ref to the Airline actor
    pub addr_airline: Addr<Airline>,
    // A vector with every request to be handled
    pub pending_requests: Vec<NewRequest>,
}

impl Actor for AirlineManager {
    type Context = Context<Self>;
}

/// A new request to handle
#[derive(Message)]
#[rtype(result = "()")]
pub struct NewRequest {
    pub info_flight: InfoFlight,
}

impl Handler<NewRequest> for AirlineManager {
    type Result = ();

    /// The NewRequest handler works as a P in a semaphore
    fn handle(&mut self, msg: NewRequest, _ctx: &mut Self::Context) -> Self::Result {
        let webservice = self
            .requests_per_airline
            .entry(msg.info_flight.flight_reservation.airline.clone())
            .or_insert((0, 0));
        let rate_limit = webservice.0;
        let current_amount_requests = webservice.1;

        if rate_limit > current_amount_requests {
            self.requests_per_airline.insert(
                msg.info_flight.flight_reservation.airline.clone(),
                (rate_limit, current_amount_requests + 1),
            );
            let _ = self.addr_airline.try_send(msg.info_flight);
        } else {
            self.pending_requests.push(msg);
        }
    }
}

/// A request that has finished
#[derive(Message)]
#[rtype(result = "()")]
pub struct FinishRequest {
    pub info_flight: InfoFlight,
}

impl Handler<FinishRequest> for AirlineManager {
    type Result = ();

    /// The FinishRequest handler works as a V in a semaphore
    fn handle(&mut self, msg: FinishRequest, ctx: &mut Self::Context) -> Self::Result {
        let webservice = self
            .requests_per_airline
            .entry(msg.info_flight.flight_reservation.airline.clone())
            .or_insert((0, 0));
        let rate_limit = webservice.0;
        let current_amount_requests = webservice.1;

        self.requests_per_airline.insert(
            msg.info_flight.flight_reservation.airline,
            (rate_limit, current_amount_requests - 1),
        );

        if !self.pending_requests.is_empty() {
            let next_request = self.pending_requests.pop().expect("No pending requests");
            ctx.notify(next_request);
        }
    }
}

/// Create an Airline actor and an AirlineManager actor
///
/// This function creates a HashMap of every airline and their rate limit, and uses it to initialize the AirlineManager
pub fn from_file(filename: &str, addr_statistics: Addr<StatsActor>) -> Addr<AirlineManager> {
    let airlines = read_file(filename).expect("Couldn't read airline file");
    let mut airline_map = HashMap::<String, (u64, u64)>::new();
    for airline in airlines {
        airline_map.insert(
            airline[0].to_string(),
            (airline[1].parse::<u64>().unwrap(), 0),
        );
    }
    let addr_airline = Airline { addr_statistics }.start();
    AirlineManager {
        requests_per_airline: airline_map,
        addr_airline,
        pending_requests: Vec::new(),
    }
    .start()
}
