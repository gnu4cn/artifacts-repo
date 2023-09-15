use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use colored::{Colorize, control};
use chrono::Local;

// Path to log file
const LOG_PATH: &str = "/home/lenny.peng/.local/log/ServerLogs.log";

/// List of different types of log headers.
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}

/// Logs a message to the console.
pub fn log(header: Header, message: &str) {
    // Type of message to log
    let header = match header {
        Header::SUCCESS => "SUCCESS".bold().bright_green(),
        Header::INFO => "INFO".bold().bright_blue(),
        Header::WARNING => "WARNING".bold().bright_yellow(),
        Header::ERROR => "ERROR".bold().bright_red()
    };

    // Print the log to the console
    // control::set_virtual_terminal(true).unwrap();
    println!("[{}] {} {}",
        Local::now().format("%m-%d-%Y %H:%M:%S").to_string().bold(),
        header,
        message
    );

    // Write the log to a file
    if Path::new(LOG_PATH).exists() {
        let mut log_file = OpenOptions::new()
            .append(true)
            .open(LOG_PATH)
            .unwrap();

        writeln!(log_file, "[{}] {} {}",
            Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            header.clear(),
            message
        ).unwrap();
    }
    else {
        let mut log_file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(LOG_PATH)
            .unwrap();

        writeln!(log_file, "[{}] {} {}",
            Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            header.clear(),
            message
        ).unwrap();
    }
}
