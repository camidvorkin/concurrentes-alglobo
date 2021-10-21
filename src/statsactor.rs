use actix::prelude::*;
use std::collections::HashMap;

pub struct Stat {
    pub elapsed_time: u128,
    pub destination: String
}

/// Actor
pub struct StatsActor {
    pub sum_time: i64,
    pub destinations: HashMap<String, i64>,
}

impl Message for Stat {
    type Result = ();
}

/// Declare actor and its context
impl Actor for StatsActor {
    type Context = Context<Self>;
}

/// Handler for `Stat` message
impl Handler<Stat> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: Stat, _: &mut Context<Self>) -> Self::Result {
        self.sum_time += msg.elapsed_time as i64;
        let mut sum_destinations = self.destinations.entry(msg.destination.clone()).or_insert(0);
        *sum_destinations += msg.elapsed_time as i64;
        self.destinations.insert(msg.destination.clone(), *sum_destinations);
    }

}
