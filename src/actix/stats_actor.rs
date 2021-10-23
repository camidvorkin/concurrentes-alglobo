use actix::prelude::*;
use common::flight_reservation::FlightReservation;
use std::collections::HashMap;

pub const STATS_FREQUENCY: i64 = 5;

pub struct StatsActor {
    sum_time: i64,
    destinations: HashMap<String, i64>,
    flights: HashMap<i32, i32>,
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

    fn get_total_count(&self) -> i64 {
        let mut count = 0;
        for (_k, v) in self.destinations.iter() {
            count += v;
        }
        count
    }

    fn get_sum_time(&self) -> i64 {
        self.sum_time
    }

    fn get_avg_time(&self) -> f64 {
        let count = self.get_total_count();
        if count == 0 {
            return 0.0;
        };
        (self.sum_time / count) as f64
    }

    fn get_top_destinations(&self, n: usize) -> Vec<(String, i64)> {
        let mut top_destinations = self
            .destinations
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i64)>>();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        top_destinations.into_iter().take(n).collect()
    }

    fn add_stat(&mut self, elapsed_time: u128, destination: String) {
        self.sum_time += elapsed_time as i64;
        let sum_destinations = self.destinations.entry(destination).or_insert(0);
        *sum_destinations += 1;
    }

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
pub struct Stat {
    pub elapsed_time: u128,
    pub flight_reservation: FlightReservation,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FinishMessage;

impl Handler<Stat> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: Stat, _ctx: &mut Self::Context) -> Self::Result {
        let responses_count = self.flights.entry(msg.flight_reservation.id).or_insert(0);
        *responses_count += 1;

        if (msg.flight_reservation.hotel && *responses_count == 2)
            || (!msg.flight_reservation.hotel && *responses_count == 1)
        {
            self.add_stat(msg.elapsed_time, msg.flight_reservation.get_route());
            self.flights.remove(&msg.flight_reservation.id);
        }

        if self.get_total_count() > 0 && (self.get_total_count() % STATS_FREQUENCY) == 0 {
            self.print_operational_stats();
            self.print_top_routes(3);
        }
    }
}

impl Handler<FinishMessage> for StatsActor {
    type Result = ();

    fn handle(&mut self, _msg: FinishMessage, _ctx: &mut Self::Context) -> Self::Result {}
}
