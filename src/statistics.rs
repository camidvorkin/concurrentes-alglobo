

use std::collections::HashMap;


pub struct Statistics {
    sum_time: f64,
    count_flights: u64,
    destinations: HashMap<String, i64>,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            sum_time: 0.0,
            count_flights: 0,
            destinations: HashMap::<String, i64>::new(),
        }
    }

    fn add_sum_time(&mut self, time: f64) {
        self.sum_time += time;
        self.count_flights += 1;
    }

    fn add_destination(&mut self, flight_code: String) {
        let flight_key = flight_code.clone();
        self.destinations.insert(flight_key, self.destinations.get(&flight_code).unwrap_or(&0) + 1);
    }

    pub fn add_reservation(&mut self, flight_code: String, time: f64) {
        self.add_sum_time(time);
        self.add_destination(flight_code);
    }

    pub fn get_avg_time(&self) -> f64 {
        return self.sum_time / self.count_flights as f64;
    }

    pub fn get_destinations(&self, flight_code: String) -> i64 {
        return *self.destinations.get(&flight_code).unwrap_or(&0);
    }

    pub fn get_top_destinations(&self, n: u64) -> Vec<(&String, &i64)> {
        let mut top_destinations: Vec<(&String, &i64)> = self.destinations.iter().collect();
        top_destinations.sort_by(|a, b| b.1.cmp(&a.1));
        return top_destinations.into_iter().take(n as usize).collect();
    }
}
