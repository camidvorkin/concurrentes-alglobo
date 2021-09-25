use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;
mod flight_reservation;
use crate::flight_reservation::FlightReservation;

fn read_file(filename: &str) -> Result<Vec<FlightReservation>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result: Vec<FlightReservation> = Vec::new();
    for line in contents.lines() {
        let flight: Vec<String>  = line.split(",").map(|x| x.to_string()).collect();
        // todo
        // let (origen, destino, airline, hotel): Tuple = line.split(",").map(|x| x.to_string()).collect();
        let flight_reservation = FlightReservation::new(flight[0].clone(), flight[1].clone(), flight[2].clone(), flight[3].clone() == "vuelo");
        result.push(flight_reservation);
    }
    Ok(result)
}

fn main() {
    println!("Hello, world and d*!");

    let filename = match env::args().nth(1) {
        Some(val) => val,
        None => "test/test.txt".to_string(),
    };


    let flights = read_file(&filename).unwrap();
    for flight_reservation in flights {
        flight_reservation.send_to_airlane();
    }
}
