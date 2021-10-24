use common::logger::{LogLevel, LoggerMsg};

use crate::statistics::Statistics;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::Sender;

/// Possible command strings that trigger the exit action
const QUIT_COMMANDS: [&str; 2] = ["Q", "QUIT"];

/// Possible command strings that trigger the show stats action
const STAT_COMMANDS: [&str; 2] = ["S", "STATS"];

/// Listents to `s` (show stats) and `q` (quit) commands
pub fn keyboard_loop(statistics: Statistics, logger_sender: &Sender<LoggerMsg>) {
    println!("At any time press S to query for stats or Q to gracefully exit \n");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let input = &*line.trim().to_uppercase();
                if QUIT_COMMANDS.contains(&input) {
                    logger_sender
                        .send(("Keyboard received QUIT command".to_string(), LogLevel::INFO))
                        .expect("Logger mpsc not receving messages");
                    return;
                } else if STAT_COMMANDS.contains(&input) {
                    logger_sender
                        .send((
                            "Keyboard received STAT command".to_string(),
                            LogLevel::TRACE,
                        ))
                        .expect("Logger mpsc not receving messages");

                    statistics.print_operational_stats();
                    statistics.print_top_routes(10);
                }
            }
            Err(_) => panic!("Failed to read stdin"),
        }
    }
}
