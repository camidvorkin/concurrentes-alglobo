use std::env;
mod flight_reservation;
mod statistics;
mod utils;
mod alglobo;
use statistics::Statistics;

fn main() {
    
    // Process airlines
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => utils::AIRLINES_FILE.to_string(),
    };
    let airlines = utils::process_airlines(&filename_airline);

    // Process flight reservations
    let filename_flights = match env::args().nth(2) {
        Some(val) => val,
        None => utils::FLIGHT_RESERVATION_FILE.to_string(),
    };
    let flights = utils::process_flights(&filename_flights);
    
    let mut reservations = vec!();
    let statistics = Statistics::new();

    for flight_reservation in flights {
        let airline_rate_limit = airlines.get(&flight_reservation.get_airline()).unwrap().clone();
        reservations.push(alglobo::reserve(flight_reservation, airline_rate_limit, statistics.clone()));
    }

    for r in reservations {
        r.join().unwrap();
    }

    // Print statistics
    print!("Total count {} \n", statistics.get_total_count());
    print!("Total sum time {} \n", statistics.get_sum_time());
    for (key, value) in statistics.get_destinations().iter() {
        println!("{} -> Total count: {}", key, value);
    }
    for (k, v) in statistics.get_top_destinations(3) {
        println!("{}. Total count: {}", k, v);
    }
    print!("Avg time: {:.2} \n", statistics.get_avg_time());
}