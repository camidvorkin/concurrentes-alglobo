use actix::prelude::*;
use std::collections::HashMap;
use crate::logger;
use crate::flight_reservation::FlightReservation;

pub struct Req {
    pub flight_reservation: FlightReservation,
}

pub struct Stat {
    pub elapsed_time: u128,
    pub destination: String,
}

/// Actor
pub struct StatsActor {
    pub sum_time: i64,
    pub destinations: HashMap<String, i64>,
    pub flights: HashMap<i32, i32>
}

impl Message for Stat {
    type Result = ();
}

impl Message for Req {
    type Result = ();
}

impl Clone for StatsActor {
    fn clone(&self) -> Self {
        StatsActor {
            sum_time: self.sum_time,
            destinations: self.destinations.clone(),
            flights: self.flights.clone()
        }
    }
}

/// Declare actor and its context
impl Actor for StatsActor {
    type Context = Context<Self>;
}

/// Handler for `Stat` message
impl Handler<Stat> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: Stat, _: &mut Context<Self>) -> Self::Result {
        logger::log(format!("New stat added! Request with route {}, was executed in {} millis", msg.destination, msg.elapsed_time));
        self.sum_time += msg.elapsed_time as i64;
        let sum_destinations = self
            .destinations
            .entry(msg.destination.clone())
            .or_insert(0);
        *sum_destinations += 1;
        self.print_stat();
    }
}

impl Handler<Req> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: Req, _: &mut Context<Self>) -> Self::Result {
        let mut x = self.flights.entry(msg.flight_reservation.id).or_insert(0);
        *x += 1;
        if msg.flight_reservation.clone().hotel {
            if *x == 2 {
                logger::log(format!("{:?} ||| New stat added! Request with id {}, was executed in {} millis", self.flights, msg.flight_reservation.id, 1));
                self.print_stat();
            }
        }
        // cuando tengo ambos aca
    }
}

impl StatsActor {
    pub fn get_total_count(&self) -> i64 {
        let mut count = 0;
        for (_k, v) in self.destinations.iter() {
            count += v;
        }
        count
    }

    pub fn get_sum_time(&self) -> i64 {
        self.sum_time
    }

    pub fn get_avg_time(&self) -> f64 {
        let count = self.get_total_count();
        if count == 0 {
            return 0.0;
        };
        (self.sum_time / count) as f64
    }

    pub fn print_stat(&self) {
        print!(
            "Operational Stats \n\
        * Completed Flights: {} \n\
        * Total Waiting Time: {} \n\
        * Avg Response time: {:.2} \n\
        * Top 3: \n{} \n",
            self.get_total_count(),
            self.get_sum_time(),
            self.get_avg_time(),
            self.get_top_destinations_str(3)
        );
    }

    pub fn get_top_destinations_str(&self, n: usize) -> String {
        let mut top_destinations = self.get_top_destinations(n);
        let mut top_destinations_str = String::new();
        for (k, v) in top_destinations.iter().take(n) {
            top_destinations_str.push_str(&format!("\tFlight {}: {} \n", k, v));
        }
        top_destinations_str
    }

    pub fn get_top_destinations(&self, n: usize) -> Vec<(String, i64)> {
        let mut top_destinations = self
            .destinations
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i64)>>();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        top_destinations.into_iter().take(n).collect()
    }
}
