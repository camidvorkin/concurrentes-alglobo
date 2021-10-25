//! Actor that handles flight stats
//!
//! This actor can handle two different type of messages: a flight stat, and a shutdown message
//!
//! This shutdown message is used to shutdown the actor after every other flight in the mailbox is processed, instead of forcefully shutting it down
use actix::prelude::*;
use common::{
    flight_reservation::FlightReservation,
    logger::{self, LogLevel},
};
use std::collections::HashMap;

/// We print the stats every N flights proccesed
pub const STATS_FREQUENCY: i64 = 5;

pub struct StatsActor {
    /// Total number of seconds spent handling requests, to then calculate the average time
    sum_time: i64,
    /// Every route and the number of flights taken so that we can report the top most used
    destinations: HashMap<String, i64>,
    /// Every flight currently being run and their number of succesful requests (either to Hotel or Airline servers)
    flights: HashMap<i32, i32>,
}

impl Default for StatsActor {
    fn default() -> Self {
        Self {
            sum_time: 0,
            destinations: HashMap::new(),
            flights: HashMap::new(),
        }
    }
}

impl Actor for StatsActor {
    type Context = Context<Self>;
}

impl StatsActor {
    pub fn new() -> StatsActor {
        StatsActor {
            sum_time: 0,
            destinations: HashMap::<String, i64>::new(),
            flights: HashMap::<i32, i32>::new(),
        }
    }

    /// Returns the number of flights taken overall
    fn get_total_count(&self) -> i64 {
        let mut count = 0;
        for (_k, v) in self.destinations.iter() {
            count += v;
        }
        count
    }

    /// Returns the number of seconds spent processing
    fn get_sum_time(&self) -> i64 {
        self.sum_time
    }

    /// Returns the average response time
    fn get_avg_time(&self) -> f64 {
        let count = self.get_total_count();
        if count == 0 {
            return 0.0;
        };
        (self.sum_time / count) as f64
    }

    /// Returns the N top routes taken
    fn get_top_destinations(&self, n: usize) -> Vec<(String, i64)> {
        let mut top_destinations = self
            .destinations
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i64)>>();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        top_destinations.into_iter().take(n).collect()
    }

    /// Adds a flight to the stats entity
    fn add_stat(&mut self, elapsed_time: u128, destination: String) {
        self.sum_time += elapsed_time as i64;
        let sum_destinations = self.destinations.entry(destination).or_insert(0);
        *sum_destinations += 1;
    }

    /// Prints the operational stats (every N flights)
    fn print_operational_stats(&self) {
        println!(
            "Operational Stats \n\
                              * Completed Flights: {} \n\
                              * Total Waiting Time: {} \n\
                              * Avg Response time: {:.2} \n",
            self.get_total_count(),
            self.get_sum_time(),
            self.get_avg_time()
        );
    }

    /// Prints the top routes (every N flights)
    fn print_top_routes(&self, n: usize) {
        let top_routes = self.get_top_destinations(n);
        if !top_routes.is_empty() {
            println!("Top {} most solicited routes", top_routes.len());
            for (k, v) in top_routes {
                println!("* {} ({} flights)", k, v);
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
/// A request stat that can be either a hotel or an airline request
pub struct Stat {
    /// How much time has the request taken
    pub elapsed_time: u128,
    /// Which flight this request belongs to
    pub flight_reservation: FlightReservation,
}

/// Message to shutdown this actor
pub struct FinishMessage;

impl Message for FinishMessage {
    type Result = Result<(i64, i64, f64), ()>;
}

impl Handler<Stat> for StatsActor {
    type Result = ();

    /// Request stat handler
    ///
    /// When a flight has been entirely processed we log it's total elapsed time
    ///
    /// If the flight is a non hotel flight, then we finish it right away. If this is a hotel-flight, we make sure that it has two succesful requests registered before we finish it
    ///
    /// If N flights have been processed, we print the stats
    fn handle(&mut self, msg: Stat, _ctx: &mut Self::Context) -> Self::Result {
        let responses_count = self.flights.entry(msg.flight_reservation.id).or_insert(0);
        *responses_count += 1;

        if (msg.flight_reservation.hotel && *responses_count == 2)
            || (!msg.flight_reservation.hotel && *responses_count == 1)
        {
            self.add_stat(msg.elapsed_time, msg.flight_reservation.get_route());
            self.flights.remove(&msg.flight_reservation.id);
            logger::log(
                format!(
                    "{} | FINISH  | The flight was processed in {} millis. ",
                    msg.flight_reservation, msg.elapsed_time
                ),
                LogLevel::INFO,
            );
        }

        if self.get_total_count() > 0 && (self.get_total_count() % STATS_FREQUENCY) == 0 {
            self.print_operational_stats();
            self.print_top_routes(3);
        }
    }
}

impl Handler<FinishMessage> for StatsActor {
    type Result = Result<(i64, i64, f64), ()>;

    /// Shutdown handler that returns the total stats
    fn handle(&mut self, _msg: FinishMessage, _ctx: &mut Self::Context) -> Self::Result {
        Ok((
            self.get_total_count(),
            self.get_sum_time(),
            self.get_avg_time(),
        ))
    }
}
