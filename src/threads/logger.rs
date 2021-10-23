use std::io::prelude::*;
use std::sync::mpsc::Receiver;

pub fn logger(logger_receiver: Receiver<String>) {
    let mut log = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("alglobo.log")
        .expect("Failed to create log file");

    loop {
        let t = chrono::prelude::Local::now();
        let s = logger_receiver
            .recv()
            .expect("Logger mpsc couldn't receive message");
        println!("{}", s);
        log.write_all(format!("{:?} |  {}\n", t, s).as_bytes())
            .expect("write failed");
    }
}
