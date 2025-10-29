#[macro_export]
macro_rules! log_err {
    // forwards args to eprintln
    ($($arg:tt)*) => {
        eprint!("[LOG] ");
        eprintln!($($arg)*);
    };
}
