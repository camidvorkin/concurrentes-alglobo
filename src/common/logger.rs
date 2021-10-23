use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "alglobo.log";

pub fn init() {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILENAME)
        .expect("Failed to create log file");
    log("Start Process".to_string());
}

pub fn log(msg: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .expect("Unable to open log file");
    println!("{}", msg);
    let msg = format!("{} | {} \n", chrono::Local::now(), msg);
    file.write_all(msg.to_string().as_bytes())
        .expect("Unable to write data");
}
