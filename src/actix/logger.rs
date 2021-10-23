use std::fs::OpenOptions;
use std::io::Write;

// const FILENAME: String = format!("{}.log", chrono::Local::today().format("%Y-%m-%d"));
const FILENAME: &str = "output/log.log";

pub fn log_init() {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILENAME)
        .unwrap();
}

pub fn log(msg: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .expect("Unable to open log file");
    let msg = format!("[{}] - {} \n", chrono::Local::now(), msg);

    // Print to console
    print!("{}", msg);

    // Print to file
    file.write_all(msg.to_string().as_bytes())
        .expect("Unable to write data");
}
