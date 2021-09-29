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

    pub fn get_airline(&self) -> String {
        return self.airline.clone();
    }

    pub fn get_hotel(&self) -> bool {
        return self.hotel.clone();
    }

    // For logging purposes
    pub fn get_flight_code(&self) -> String {
        return format!("{}-{}", self.origin, self.destination);
    }
}