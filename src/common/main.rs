//! Common constants, structures and functions between both implementations
#![forbid(unsafe_code)]
pub mod flight_reservation;
pub mod logger;
pub mod simulate_requests;
pub mod utils;

/// Airlines CSV config file
pub const AIRLINES_FILE: &str = "src/configs/airlines.txt";

/// Flight CSV file to default if it wasn't specified as CLA
pub const TEST_FLIGHTS_FILE: &str = "src/test/test_flights.txt";
