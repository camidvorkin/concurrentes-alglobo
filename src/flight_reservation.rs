use crate::airline::Airline;
use crate::hotel::Hotel;
use std::thread;
use std::time::Duration;
use std::string::ToString;
pub struct FlightReservation {
    origin: String,
    destination: String,
    airline: String,
    hotel: bool,
}

impl ToString for FlightReservation {
    fn to_string(&self) -> String {
        return format!("FlightReservation: origin: {}, destination: {}, airline: {}, hotel: {}", self.origin, self.destination, self.airline, self.hotel);
    }
}

impl FlightReservation {
    pub fn new(origin: String, destination: String, airline: String, hotel: bool) -> FlightReservation {
        FlightReservation {
            origin,
            destination,
            airline,
            hotel
        }
    }

    pub fn reserve(&self, airline: &Airline, hotel: Hotel) {  
        let airline = airline.clone();
        let airline_name = self.airline.clone();
        
        let handle = thread::spawn( move || {
            loop {
                if airline.reseve() {
                    print!("Flight reservation successful for {}\n", airline_name);
                    break;
                }
                print!("Flight reservation failed for {}\n", airline_name);
                thread::sleep(Duration::from_millis(1000));
            }
        });
        if self.hotel {
            self.send_to_hotel(hotel);
        }
        handle.join().unwrap();
    }

    fn send_to_hotel(&self, hotel: Hotel) {
        hotel.reserve();
        print!("Hotel reservation successful \n");
    }

    pub fn get_airline(&self) -> String {
        return self.airline.clone();
    }

}