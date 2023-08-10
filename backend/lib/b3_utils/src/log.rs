use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::thread::LocalKey;

pub mod counter;

thread_local! {
    pub static MAIN_LOG: RefCell<LogBuffer> = RefCell::new(LogBuffer::with_capacity(1000));
}

/// Adds a new record to a canister log buffer.
///
/// ```
/// use b3_utils::{log, export_log};
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
/// ```
#[macro_export]
macro_rules! log {
    ($message:expr $(,$args:expr)* $(,)*) => {{
        use $crate::{MAIN_LOG, Sink};
        let message = std::format!($message $(,$args)*);
        // Print the message for convenience for local development (e.g. integration tests)
        println!("{}", &message);
        (&MAIN_LOG).append($crate::LogEntry {
            timestamp: $crate::now(),
            message,
            file: std::file!(),
            line: std::line!(),
            counter: $crate::counter::increment()
        });
    }}
}

/// An entry in the canister log.
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogEntry {
    pub timestamp: u64,
    // The index of this entry starting from the last canister upgrade.
    pub counter: u64,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{} {}",
            self.timestamp, self.file, self.line, self.message
        )
    }
}

/// A circular buffer for log messages.
pub struct LogBuffer {
    max_capacity: usize,
    entries: VecDeque<LogEntry>,
}

impl LogBuffer {
    /// Creates a new buffer of the specified max capacity.
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            entries: VecDeque::with_capacity(max_capacity),
        }
    }

    /// Adds a new entry to the buffer, potentially evicting older entries.
    pub fn append(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Returns an iterator over entries in the order of their insertion.
    pub fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// Returns the first iterator for which p returns false (or past the end
    /// iterator if they all return false).
    ///
    /// Warning: Entries MUST be partitioned by p. That is, p returns true for
    /// all elements in the "front" of the entries list, and false for all
    /// elements in the "back". Otherwise, behavior is not defined. This is
    /// because binary search is used.
    ///
    /// For example,
    ///
    ///   log_buffer.skip_old_entries(|log_entry| log_entry.timestamp <= T)
    ///
    /// In practice, p only uses the timestamp field, because you can partition
    /// on that (since entries are in chronological order, assuming the clock is
    /// monotonic, and the IC, it is).
    ///
    /// If you want an iterator to the first iterator that returns true, but p
    /// does not partition, do this instead:
    ///
    ///    log_buffer.iter().skip_while(opposite_of_p)
    pub fn entries_partition_point<P>(&self, p: P) -> impl Iterator<Item = &LogEntry>
    where
        P: Fn(&LogEntry) -> bool,
    {
        let head_len = self.entries.partition_point(p);
        self.iter().skip(head_len)
    }
}

pub type GlobalBuffer = LocalKey<RefCell<LogBuffer>>;

#[derive(Clone)]
pub struct DevNull;

pub trait Sink {
    fn append(&self, entry: LogEntry);
}

impl Sink for DevNull {
    fn append(&self, _: LogEntry) {}
}

impl Sink for &'static GlobalBuffer {
    fn append(&self, entry: LogEntry) {
        self.with(|cell| cell.borrow_mut().append(entry))
    }
}

mod private {
    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "ic0")]
    extern "C" {
        pub fn time() -> u64;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn timestamp() -> u64 {
        unsafe { time() }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn timestamp() -> u64 {
        use std::time::SystemTime;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_nanos() as u64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}

/// Returns the current time as a number of nanoseconds passed since the Unix
/// epoch.
#[doc(hidden)]
pub fn now() -> u64 {
    private::timestamp()
}

/// Exports the contents of a buffer as a vector of entries in the order of
/// insertion.
///
/// ```
/// use b3_utils::{export_log, log};
///
/// log!("Hello, {}!", "world");
/// let entries = export_log();
/// assert_eq!(entries.len(), 1);
/// assert_eq!(entries[0].message, "Hello, world!");
/// ```
pub fn export_log() -> Vec<LogEntry> {
    MAIN_LOG.with(|log| log.borrow().iter().cloned().collect())
}

#[cfg(test)]
mod test {
    use crate::{export_log, log};

    #[test]
    fn test_log() {
        log!("Hello, {}!", "world");
        let entries = export_log();

        println!("{}", entries[0]);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].counter, 1);
        assert_eq!(entries[0].message, "Hello, world!");
    }
}
