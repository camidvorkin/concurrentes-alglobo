use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;
use std::thread; 
use rand::Rng;
use std::time::Duration;
use std::sync::{Arc, Barrier};
use std::collections::HashMap;
use std_semaphore::Semaphore;

mod flight_reservation;
mod statistics;
use statistics::Statistics;
use crate::flight_reservation::FlightReservation;

const PACKAGE: &str = "package";

// Read file and return split content
fn read_file(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::<Vec<String>>::new();
    for line in contents.lines() {
        let flight: Vec<String>  = line.split(",").map(|x| x.to_string()).collect();
        result.push(flight);
      } 
    Ok(result)
}

// Process Airlines and set rate limiting to Semaphore
fn process_airlines(filename: &str) -> HashMap<String, Arc<Semaphore>> {
    let airlines = read_file(filename).unwrap();
    let mut airline_map = HashMap::<String, Arc<Semaphore>>::new();
    for airline in airlines {
        airline_map.insert(airline[0].clone(), Arc::new(Semaphore::new(airline[1].parse::<isize>().unwrap())));
    }
    return airline_map;
}

// Process Flights to set FlightReservation struct
fn process_flights(filename: &str) -> Vec<FlightReservation> {
    let flights_reservations = read_file(filename).unwrap();
    let mut flights = Vec::<FlightReservation>::new();
    for flight in flights_reservations {
        flights.push(FlightReservation::new(flight[0].clone(), flight[1].clone(), flight[2].clone(), flight[3].clone() == PACKAGE))
    }
    flights
}

// Request to hotels
fn send_to_hotel() -> () {
    thread::sleep(Duration::from_secs(1));
    print!("Hotel reservation successful \n");
}

// Request to airline. Returns true if request was accepted, false otherwise
fn _send_to_airline() -> bool {
    thread::sleep(Duration::from_secs(1));
    let rng = rand::thread_rng().gen_bool(0.8);
    return rng;
}

// Request flight
fn reserve(flight_reservation: FlightReservation, rate_limit:Arc<Semaphore>, mut statistics: Statistics) -> thread::JoinHandle<()> {
    let airline_name = flight_reservation.get_airline();
    let flight_code = flight_reservation.get_flight_code();
    let flight_destination = flight_reservation.get_destination();
    let flight_origin = flight_reservation.get_origin();
    let sem = rate_limit.clone();

    let barrier_c1 = Arc::new(Barrier::new(3));
    let barrier_c2 = barrier_c1.clone();
    let barrier_c3 = barrier_c1.clone();

    sem.acquire();

    let start_time = std::time::Instant::now();
    
    // Send request to the airline and hotel(if was requested) simultaneously
    let _handle1 = thread::spawn( move || { 
        if flight_reservation.get_hotel() {
            send_to_hotel();
        }
        barrier_c1.wait();
    });
    
    let _handle2 = thread::spawn( move || {
        loop {
            if _send_to_airline() {
                print!("Flight reservation successful for {}. For flight: {} \n", airline_name, flight_code);
                sem.release();
                barrier_c2.wait();
                break;
            }
            print!("Flight reservation failed for {}. For flight: {} \n", airline_name, flight_code);
            thread::sleep(Duration::from_millis(5000));
        }
    });

    let handle3 = thread::spawn( move || {
        barrier_c3.wait();
        statistics.add_flight_reservation(start_time, (flight_origin, flight_destination));
    });
    return handle3;
}

fn main() {
    // Process airlines
    let filename_airline = match env::args().nth(1) {
        Some(val) => val,
        None => "src/configs/airlines.txt".to_string(),
    };
    // Todo: airline factory estatico / global / algo
    let airline_factory = process_airlines(&filename_airline);

    // Process flight reservations
    let filename = match env::args().nth(2) {
        Some(val) => val,
        None => "test/test.txt".to_string(),
    };
    let flights = process_flights(&filename);
    let mut reservations = vec!();
    let statistics = Statistics::new();

    for flight_reservation in flights {
        let airline_rate_limit = airline_factory.get(&flight_reservation.get_airline()).unwrap().clone();
        reservations.push(reserve(flight_reservation, airline_rate_limit, statistics.clone()));
    }

    for r in reservations {
        r.join().unwrap();
    }

    // Print statistics
    print!("Total count {} \n", statistics.get_total_count());
    print!("Total sum time {} \n", statistics.get_sum_time());
    for (key, value) in statistics.get_destinations().iter() {
        println!("Orgin: {:?} -> Destination: {:?} -> Total count: {}", key.0, key.1, value);
    }
    for (k, v) in statistics.get_top_destinations(3) {
        println!("Origin: {} -> Destination: {}. Total count: {}", k.0, k.1, v);
    }
    print!("Avg time: {:.2} \n", statistics.get_avg_time());
}