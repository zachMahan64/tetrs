#[macro_export]
macro_rules! log_err {
    // forwards args to eprintln
    ($($arg:tt)*) => {
        eprint!("[LOG] ");
        eprintln!($($arg)*);
    };
}

use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tetrs.log")
        .expect("failed to open log file");
    writeln!(file, "{}", msg).expect("failed to write log");
}
