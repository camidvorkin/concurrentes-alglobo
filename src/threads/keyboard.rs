use crate::statistics::Statistics;
use std::io;
use std::io::prelude::*;

/// Possible command strings that trigger the exit action
const QUIT_COMMANDS: [&str; 2] = ["Q", "QUIT"];

/// Possible command strings that trigger the show stats action
const STAT_COMMANDS: [&str; 2] = ["S", "STATS"];

/// Listents to `s` (show stats) and `q` (quit) commands
pub fn keyboard_listener(statistics: Statistics) {
    println!("At any time press S to query for stats or Q to gracefully exit \n");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let input = &*line.trim().to_uppercase();
                if QUIT_COMMANDS.contains(&input) {
                    // TODO: como puedo matar a un thread desde si mismo?
                    // o mandar una señal a main para que lo mate
                    println!("quit");
                    break;
                } else if STAT_COMMANDS.contains(&input) {
                    statistics.print_operational_stats();
                    statistics.print_top_routes(10);
                }
            }
            Err(_) => panic!("Failed to read stdin"),
        }
    }
}
