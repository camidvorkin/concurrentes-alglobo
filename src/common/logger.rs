use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "alglobo.log";
#[derive(Debug)]
pub enum LogLevel {
    TRACE,
    INFO,
}

pub type LoggerMsg = (String, LogLevel);

pub fn init() {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILENAME)
        .expect("Failed to create log file");
    log("START".to_string(), LogLevel::TRACE);
}

pub fn log(msg: String, loglevel: LogLevel) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .expect("Unable to open log file");
    if let LogLevel::INFO = loglevel {
        println!("{}", msg)
    };
    // We want to have the loglevel on exactly 5 characters, so that `| TRACE |` and `| INFO  |` have the same width.
    // This formatting only works with strings, not debug strings
    // i.e. {:^7} works, but {:^7?} does not
    // So we first do some format! shenanigans to convert the debug string to a string
    let loglevelstr = format!("{:?}", loglevel);

    let msg = format!("{} | {:^5} | {} \n", chrono::Local::now(), loglevelstr, msg);
    file.write_all(msg.as_bytes())
        .expect("Unable to write data");
}
