#![macro_use] 
/// Logging macros for custom log levels.
///
/// Export an environment variable with the desired logging verbosity: 
///
/// ```bash
/// export RUST_LOG_LEVEL=4
/// ```
/// 
/// ## Log Levels ##
/// 
/// - trace: 5 *(future)*
/// - debug: 4
/// - info: 3 
/// - warn: 2 *(future)*
/// - error: 1 *(future)*
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate qlearn;
/// # fn main() {
/// debug!("This is a debug message: {}.", 37);
/// # }
/// ```

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => (
        let log_level = ::std::env::var("RUST_LOG_LEVEL");
        if log_level.is_ok() && log_level.clone().unwrap().parse::<i32>().is_ok() {
            if log_level.unwrap().parse::<i32>().unwrap() >= 4 { 
                (print!(concat!($fmt, "\n"))) 
            }
        }
    );
    ($fmt:expr, $($arg:tt)*) => (
        let log_level = ::std::env::var("RUST_LOG_LEVEL");
        if log_level.is_ok() && log_level.clone().unwrap().parse::<i32>().is_ok() {
            if log_level.unwrap().parse::<i32>().unwrap() >= 4 { 
                (print!(concat!($fmt, "\n"), $($arg)*))
            }
        }
    )
}

#[macro_export]
macro_rules! info {
    ($fmt:expr) => (
        let log_level = ::std::env::var("RUST_LOG_LEVEL");
        if log_level.is_ok() && log_level.clone().unwrap().parse::<i32>().is_ok() {
            if log_level.unwrap().parse::<i32>().unwrap() >= 3 { 
                (print!(concat!($fmt, "\n"))) 
            }
        }
    );
    ($fmt:expr, $($arg:tt)*) => (
        let log_level = ::std::env::var("RUST_LOG_LEVEL");
        if log_level.is_ok() && log_level.clone().unwrap().parse::<i32>().is_ok() {
            if log_level.unwrap().parse::<i32>().unwrap() >= 3 { 
                (print!(concat!($fmt, "\n"), $($arg)*))
            }
        }
    )
}
