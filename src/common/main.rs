//! Common constants, structures and functions between both implementations
#![forbid(unsafe_code)]
pub mod flight_reservation;
pub mod logger;
pub mod utils;

/// Airlines CSV config file
pub const AIRLINES_FILE: &str = "src/configs/airlines.txt";

/// Flight CSV file to default if it wasn't specified as CLA
pub const TEST_FLIGHTS_FILE: &str = "src/test/test_flights.txt";

/// The minimum amount of seconds a simulated request takes
pub const MIN_TIME: u64 = 1;
/// The maximum amount of seconds a simulated request takes
pub const MAX_TIME: u64 = 3;
