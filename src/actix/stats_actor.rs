//! Actor that handles flight stats
//!
//! It's initialized with the total number of flights to be processed, to know when to shut down
//!
//! It receives completed requests, and when a flight has all of their requests completed, it logs it as finished
//!
//! Every N completed flights, it prints out some operational and domain-specific stats
use actix::prelude::*;
use common::{
    flight_reservation::FlightReservation,
    logger::{self, LogLevel},
};
use std::collections::HashMap;

/// We print the stats every N flights proccesed
pub const STATS_FREQUENCY: i64 = 5;

/// Actor struct
pub struct StatsActor {
    /// Total number of seconds spent handling requests, to then calculate the average time
    sum_time: i64,
    /// Every route and the number of flights taken so that we can report the top most used
    destinations: HashMap<String, i64>,
    /// Every flight currently being run (i.e, waiting for either a hotel or an airline request) and their number of succesful requests
    flights: HashMap<i32, i32>,
    /// Number of flights to be processed in total
    flights_to_process: usize,
}

impl Actor for StatsActor {
    type Context = Context<Self>;
}

impl StatsActor {
    pub fn new(n: usize) -> StatsActor {
        StatsActor {
            sum_time: 0,
            destinations: HashMap::<String, i64>::new(),
            flights: HashMap::<i32, i32>::new(),
            flights_to_process: n,
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
/// A request stat that represents either a hotel or an airline request
pub struct Stat {
    /// How much time has the request taken
    pub elapsed_time: u128,
    /// Which flight this request belongs to
    pub flight_reservation: FlightReservation,
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
    ///
    /// If all flights have been processed, we shut down the system
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

        if self.get_total_count() == self.flights_to_process as i64 {
            System::current().stop();
        }
    }
}
