use std::cell::RefCell;

use super::{LogBuffer, LogEntry};

thread_local! {
    pub static MAIN_LOG: RefCell<LogBuffer> = RefCell::new(LogBuffer::with_capacity(100));
}

pub fn with_log<F, R>(f: F) -> R
where
    F: FnOnce(&LogBuffer) -> R,
{
    MAIN_LOG.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_log_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut LogBuffer) -> R,
{
    MAIN_LOG.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}

/// Adds a new record to a canister log buffer.
///
/// ```
/// use b3_utils::{log, logs::export_log};
///
/// // Keep up to 1000 last messages.
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
            message,
            file: std::file!(),
            line: std::line!(),
            counter: $crate::logs::counter::log_increment()
        });
    }}
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion by page.
///
/// ```
/// use b3_utils::{log, logs::export_log_page};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log_page(0, None);
/// assert_eq!(entries.len(), 1);
/// assert_eq!(entries[0].message, "Hello, world!");
/// ```
pub fn export_log_page(page: u32, page_size: Option<u32>) -> Vec<LogEntry> {
    let page_size = page_size.unwrap_or(100);

    with_log(|log| {
        let start = page * page_size;
        let end = start + page_size;
        log.iter()
            .skip(start as usize)
            .take(end as usize)
            .cloned()
            .collect()
    })
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion.
///
/// ```
/// use b3_utils::{log, logs::export_log};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log();
/// assert_eq!(entries.len(), 1);
/// assert_eq!(entries[0].message, "Hello, world!");
/// ```
pub fn export_log() -> Vec<LogEntry> {
    with_log(|log| log.iter().cloned().collect())
}

/// Exports the contents of messages vector of entries in the order of
/// insertion by page.
///
/// ```
/// use b3_utils::{log, logs::export_log_messages_page};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log_messages_page(0, None);
///
/// assert_eq!(entries.len(), 1);
/// ```
pub fn export_log_messages_page(page: u32, page_size: Option<u32>) -> Vec<String> {
    let page_size = page_size.unwrap_or(100);

    with_log(|log| {
        let start = page * page_size;
        let end = start + page_size;
        log.iter()
            .skip(start as usize)
            .take(end as usize)
            .map(|entry| entry.to_string())
            .collect()
    })
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion.
///
/// ```
/// use b3_utils::{log, logs::export_log_messages};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log_messages();

/// assert_eq!(entries.len(), 1);
/// ```
pub fn export_log_messages() -> Vec<String> {
    with_log(|log| log.iter().map(|entry| entry.to_string()).collect())
}
