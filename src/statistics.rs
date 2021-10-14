

use std::collections::HashMap;
use std::sync::{Arc, RwLock};


pub struct Statistics {
    sum_time: Arc<RwLock<i64>>,
    count_flights: Arc<RwLock<i64>>,
    destinations: Arc<RwLock<HashMap<String , i64>>>,
}

// Impl clone
impl Clone for Statistics {
    fn clone(&self) -> Self {
        Statistics {
            sum_time: self.sum_time.clone(),
            count_flights: self.count_flights.clone(),
            destinations: self.destinations.clone(),
        }
    }
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            sum_time: Arc::new(RwLock::new(0)),
            count_flights: Arc::new(RwLock::new(0)),
            destinations: Arc::new(RwLock::new(HashMap::<String, i64>::new())),
        }
    }

    pub fn add_flight_reservation(&mut self, start_time: std::time::Instant, destination: String) {
        {
            // Add one flight
            let mut count = self.count_flights.write().unwrap();
            *count += 1;

            // Sum time elapsed since flight was processed
            let diff = start_time.elapsed().as_millis() as i64;
            let mut sum_time = self.sum_time.write().unwrap();
            *sum_time += diff;

            // Add (origin, destination)
            let mut map = self.destinations.write().expect("RwLock poisoned");
            *map.entry(destination).or_insert(0) += 1;
        }
    }

    pub fn get_total_count(&self) -> i64 {
        let count = self.count_flights.read().unwrap();
        *count
    }

    pub fn get_sum_time(&self) -> i64 {
        let sum_time = self.sum_time.read().unwrap();
        *sum_time
    }

    pub fn get_avg_time(&self) -> f64 {
        let sum_time = self.sum_time.read().unwrap();
        let count = self.count_flights.read().unwrap();
        if *count == 0 { return 0.0 };
        (*sum_time as f64) / (*count as f64)
    }

    pub fn get_top_destinations(&self, n: usize) -> Vec<(String, i64)> {
        let map = self.destinations.read().unwrap();
        let mut top_destinations = map.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, i64)>>();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        top_destinations.into_iter().take(n).collect()
    }
}
