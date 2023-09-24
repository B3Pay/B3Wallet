use std::cell::RefCell;

use super::{buffer::LogBuffer, LogEntry};

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

/// Imports a vector of entries into a buffer.
/// The entries are inserted in the order of the vector.
/// Older entries are evicted.
///
/// # Example
/// ```
/// use b3_utils::{logs::{import_log, LogEntry}, log};
///
/// log!("Hello, {}!", "world");
/// let entries = import_log(vec![LogEntry {
///     timestamp: b3_utils::NanoTimeStamp::now(),
///     message: "Hello, log!".to_string(),
///     file: "src/logs.rs",
///     variant: b3_utils::logs::LogVariant::Info,
///     line: 123,
///     cycle: None,
///     version: env!("CARGO_PKG_VERSION"),
///     counter: 1,
/// }]);
/// assert_eq!(entries.len(), 2);
/// assert_eq!(entries[0].message, "Hello, world!");
/// assert_eq!(entries[1].message, "Hello, log!");
/// ```
pub fn import_log(entries: Vec<LogEntry>) -> Vec<LogEntry> {
    with_log_mut(|log| {
        log.set_capacity(100);
        for entry in entries {
            log.append(entry);
        }
        log.export()
    })
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion.
///
/// ```
/// use b3_utils::{logs::export_log, log};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log();
/// assert_eq!(entries.len(), 1);
/// assert_eq!(entries[0].message, "Hello, world!");
/// ```
pub fn export_log() -> Vec<LogEntry> {
    with_log(|log| log.export())
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion by page.
///
/// ```
/// use b3_utils::{logs::export_log_page, log};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log_page(0, None);
/// assert_eq!(entries.len(), 1);
/// assert_eq!(entries[0].message, "Hello, world!");
/// ```
pub fn export_log_page(page: usize, page_size: Option<usize>) -> Vec<LogEntry> {
    let page_size = page_size.unwrap_or(100);

    with_log(|log| log.export_page(page, page_size))
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
pub fn export_log_messages_page(page: usize, page_size: Option<usize>) -> Vec<String> {
    let page_size = page_size.unwrap_or(100);

    with_log(|log| log.export_messages_page(page, page_size))
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
    with_log(|log| log.export_messages())
}
