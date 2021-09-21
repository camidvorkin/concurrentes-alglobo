
pub struct FlightReservation {
    origin: String,
    destination: String,
    airline: String,
    hotel: bool,
}

// Create a map of reservations
// pub fn create_reservations() -> HashMap<String, FlightReservation> {
//     let mut reservations: HashMap<String, FlightReservation> = HashMap::new();
//     reservations.insert("Airlane", Airlane::new());
//     reservations.insert("Iberia", Iberia::new());
//     reservations.insert("Aereolineas Argentina", Aereo::new());
// }

impl FlightReservation {
    pub fn new(origin: String, destination: String, airline: String, hotel: bool) -> FlightReservation {
        FlightReservation {
            origin,
            destination,
            airline,
            hotel
        }
    }

    pub fn saludar(&self) {
        println!("Hola, soy una reserva de vuelo de {} a {}", self.origin, self.destination);
        println!("Con la aereoliea {} en {}", self.airline, self.hotel);
    }

    pub fn send_to_airlane(&self) {
    }
}

pub enum PackageType {
    HOTEL,
    COMPLETE,
}