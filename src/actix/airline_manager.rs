use crate::airline::Airline;
use crate::info_flight::InfoFlight;
use crate::stats_actor::StatsActor;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use common::utils::read_file;
use std::collections::HashMap;

pub struct AirlineManager {
    pub requests_per_airline: HashMap<String, (u64, u64)>,
    pub addr_airline: Addr<Airline>,
}

impl Actor for AirlineManager {
    type Context = Context<Self>;
}

pub struct NewRequest {
    pub info_flight: InfoFlight,
}

impl Message for NewRequest {
    type Result = ();
}

impl Handler<NewRequest> for AirlineManager {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, ctx: &mut Self::Context) -> Self::Result {
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
            ctx.run_later(std::time::Duration::from_millis(1), move |act, _ctx| {
                act.handle(msg, _ctx);
            });
        }
    }
}

pub struct FinishRequest {
    pub info_flight: InfoFlight,
}

impl Message for FinishRequest {
    type Result = ();
}

impl Handler<FinishRequest> for AirlineManager {
    type Result = ();

    fn handle(&mut self, msg: FinishRequest, _ctx: &mut Self::Context) -> Self::Result {
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
    }
}

/// Create an Airline actor for each available airline in file
///
/// Returns a HashMap with the name of the airline and the address of the actor.
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
    }
    .start()
}
