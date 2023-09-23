use crate::NanoTimeStamp;
use candid::CandidType;

use std::fmt;

pub mod counter;

mod store;
pub use store::*;

mod buffer;
pub use buffer::*;

mod test;

/// An entry in the canister log.
#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogEntry {
    pub timestamp: NanoTimeStamp,
    // The index of this entry starting from the last canister upgrade.
    pub cycle: Option<u128>,
    pub counter: u64,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
    pub version: &'static str,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}][{}] {}:{} {}",
            self.timestamp,
            self.cycle.unwrap_or(0),
            self.file,
            self.line,
            self.message
        )
    }
}

/// Adds a new record to a canister log buffer.
/// The maximum number of records is 1000.
/// Older records are evicted.
///
/// The log is not resilient to canister upgrades.
///
/// The log is exported by calling `export_log()`.
/// And it can be imported by calling `import_log()`.
///
/// # Example
/// ```
/// use b3_utils::{logs::export_log, log};
///
/// fn sum_and_log(x: u64, y: u64) -> u64 {
///    let result = x.saturating_add(y);
///    log!("{} + {} = {}", x, y, result);
///    result
/// }
///
/// assert_eq!(sum_and_log(1, 2), 3);
/// assert_eq!(export_log()[0].message, "1 + 2 = 3");
/// assert_eq!(export_log()[0].counter, 1);
/// ```
#[macro_export]
macro_rules! log {
    ($message:expr $(,$args:expr)* $(,)*) => {{
        use $crate::logs::Sink;
        let message = std::format!($message $(,$args)*);
        // Print the message for convenience for local development (e.g. integration tests)
        println!("{}", &message);
        (&$crate::logs::MAIN_LOG).append($crate::logs::LogEntry {
            timestamp: $crate::NanoTimeStamp::now(),
            cycle: None,
            message,
            file: std::file!(),
            line: std::line!(),
            version: env!("CARGO_PKG_VERSION"),
            counter: $crate::logs::counter::log_increment()
        });
    }}
}

#[macro_export]
macro_rules! log_panic {
    ($message:expr $(,$args:expr)* $(,)*) => {{
        use $crate::logs::Sink;
        let message = std::format!($message $(,$args)*);
        // Print the message for convenience for local development (e.g. integration tests)
        (&$crate::logs::MAIN_LOG).append($crate::logs::LogEntry {
            timestamp: $crate::NanoTimeStamp::now(),
            cycle: None,
            message: message.clone(),
            file: std::file!(),
            line: std::line!(),
            version: env!("CARGO_PKG_VERSION"),
            counter: $crate::logs::counter::log_increment()
        });
        panic!("{}", &message);
    }}
}
/// Adds a new record to a canister log buffer.
/// The maximum number of records is 1000.
/// Older records are evicted.
///
/// The log is not resilient to canister upgrades.
///
/// The log is exported by calling `export_log()`.
/// And it can be imported by calling `import_log()`.
///
/// # Example
/// ```
/// use b3_utils::{logs::export_log, log_cycle};
///
/// fn sum_and_log(x: u64, y: u64) -> u64 {
///    let result = x.saturating_add(y);
///    log_cycle!("{} + {} = {}", x, y, result);
///    result
/// }
///
/// assert_eq!(sum_and_log(1, 2), 3);
/// assert_eq!(export_log()[0].message, "1 + 2 = 3");
/// assert_eq!(export_log()[0].counter, 1);
/// ```
#[macro_export]
macro_rules! log_cycle {
    ($message:expr $(,$args:expr)* $(,)*) => {{
        use $crate::logs::Sink;

        #[cfg(not(target_arch = "wasm32"))]
        use $crate::mocks::canister_balance_mock as canister_balance;
        #[cfg(target_arch = "wasm32")]
        use ic_cdk::api::canister_balance128 as canister_balance;

        let message = std::format!($message $(,$args)*);
        // Print the message for convenience for local development (e.g. integration tests)
        println!("{}", &message);
        (&$crate::logs::MAIN_LOG).append($crate::logs::LogEntry {
            timestamp: $crate::NanoTimeStamp::now(),
            cycle: Some(canister_balance()),
            message,
            file: std::file!(),
            line: std::line!(),
            version: env!("CARGO_PKG_VERSION"),
            counter: $crate::logs::counter::log_increment()
        });
    }}
}
/// Adds a new record to a canister log buffer and returns an error.
/// This macro is useful for early returns from a function.
///
/// # Example
/// ```
/// use b3_utils::{logs::export_log, require_log};
///
/// fn sum_and_log(x: u64, y: u64) -> Result<u64, String> {
///     let result = x.saturating_add(y);
///     require_log!(result < 100, "Result is too big: {}", result);
///
///     Ok(result)
/// }
///
/// assert_eq!(sum_and_log(1, 2), Ok(3));
///
/// match sum_and_log(100, 2) {
///     Ok(_) => panic!("Should have failed"),
///     Err(e) => assert_eq!(e, "Result is too big: 102"),
/// }
///
/// assert_eq!(export_log()[0].message, "Result is too big: 102");
/// assert_eq!(export_log()[0].counter, 1);
/// ```
#[macro_export]
macro_rules! require_log {
    ($condition:expr, $($msg:tt)*) => {
        if !$condition {
            $crate::log!($($msg)*);

            return $crate::report(format!($($msg)*));
        }
    };
}

#[cfg(test)]
mod test_utils {
    use crate::{logs::export_log, require_log};

    #[test]
    fn test_log() {
        fn sum_and_log(x: u64, y: u64) -> Result<u64, String> {
            let result = x.saturating_add(y);
            require_log!(result < 100, "Result is too big: {}", result);

            Ok(result)
        }

        assert_eq!(sum_and_log(1, 2), Ok(3));

        match sum_and_log(100, 2) {
            Ok(_) => panic!("Should have failed"),
            Err(e) => assert_eq!(e, "Result is too big: 102"),
        }

        assert_eq!(export_log()[0].message, "Result is too big: 102");
        assert_eq!(export_log()[0].counter, 1);
    }
}
