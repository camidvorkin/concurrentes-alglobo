//! Flight Stats
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Entity that holds the statistics of the flights
pub struct Statistics {
    sum_time: Arc<RwLock<i64>>,
    destinations: Arc<RwLock<HashMap<String, i64>>>,
}

impl Clone for Statistics {
    fn clone(&self) -> Self {
        Statistics {
            sum_time: self.sum_time.clone(),
            destinations: self.destinations.clone(),
        }
    }
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            sum_time: Arc::new(RwLock::new(0)),
            destinations: Arc::new(RwLock::new(HashMap::<String, i64>::new())),
        }
    }

    /// Adds request time to the accumulated sum, and the flights origin->destination to the hashmap
    pub fn add_flight_reservation(&mut self, start_time: std::time::Instant, destination: String) -> i64 {
        {
            let diff = start_time.elapsed().as_millis() as i64;
            let mut sum_time = self.sum_time.write().unwrap();
            *sum_time += diff;

            let mut map = self.destinations.write().expect("RwLock poisoned");
            *map.entry(destination).or_insert(0) += 1;
            diff
        }
    }

    pub fn get_total_count(&self) -> i64 {
        let mut count = 0;
        let map = self.destinations.read().unwrap();
        for (_k, v) in map.iter() {
            count += v;
        }
        count
    }

    pub fn get_sum_time(&self) -> i64 {
        let sum_time = self.sum_time.read().unwrap();
        *sum_time
    }

    pub fn get_avg_time(&self) -> f64 {
        let sum_time = self.get_sum_time();
        let count = self.get_total_count();
        if count == 0 {
            return 0.0;
        };
        (sum_time / count) as f64
    }

    pub fn get_top_destinations(&self, n: usize) -> Vec<(String, i64)> {
        let map = self.destinations.read().unwrap();
        let mut top_destinations = map
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i64)>>();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        top_destinations.into_iter().take(n).collect()
    }

    pub fn print_operational_stats(&self) {
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

    pub fn print_top_routes(&self, n: usize) {
        let top_routes = self.get_top_destinations(n);
        if !top_routes.is_empty() {
            println!("Top {} most solicited routes", top_routes.len());
            for (k, v) in top_routes {
                println!("* {} ({} flights)", k, v);
            }
        }
    }
}
