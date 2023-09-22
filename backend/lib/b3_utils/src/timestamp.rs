use candid::CandidType;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::mocks::time_mock as ic_timestamp;
#[cfg(target_arch = "wasm32")]
use ic_cdk::api::time as ic_timestamp;

mod test;
mod traits;

#[derive(
    Default, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug,
)]
pub struct NanoTimeStamp(pub u64);

impl NanoTimeStamp {
    // Constants for nanosecond conversions
    pub const NS_PER_MILLISECOND: u64 = 1_000_000;
    pub const NS_PER_SECOND: u64 = Self::NS_PER_MILLISECOND * 1_000;
    pub const NS_PER_MINUTE: u64 = Self::NS_PER_SECOND * 60;
    pub const NS_PER_HOUR: u64 = Self::NS_PER_MINUTE * 60;
    pub const NS_PER_DAY: u64 = Self::NS_PER_HOUR * 24;

    /// Constructor function that returns the current timestamp
    pub fn now() -> Self {
        NanoTimeStamp(ic_timestamp())
    }

    pub fn days_from_now(days: u64) -> Self {
        let ns_to_add = days * Self::NS_PER_DAY;
        NanoTimeStamp(ic_timestamp() + ns_to_add)
    }

    pub fn from_le_bytes(bytes: [u8; 8]) -> Self {
        NanoTimeStamp(u64::from_le_bytes(bytes))
    }

    pub fn elapsed(&self) -> Self {
        let now = NanoTimeStamp::now();
        if self > &now {
            NanoTimeStamp(0)
        } else {
            NanoTimeStamp(now.0 - self.0)
        }
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Converts the timestamp to seconds
    pub fn to_secs(&self) -> u64 {
        self.0 / Self::NS_PER_SECOND
    }

    /// Converts the timestamp to milliseconds
    pub fn to_millis(&self) -> u64 {
        self.0 / Self::NS_PER_MILLISECOND
    }

    /// Converts the timestamp to milliseconds
    pub fn as_global_timer(&self) -> i64 {
        self.0 as i64
    }

    /// Checks if the deadline has passed
    pub fn has_passed(&self) -> bool {
        let now = NanoTimeStamp::now();
        self < &now
    }

    /// Checks if the deadline is still in the future
    pub fn in_future(&self) -> bool {
        let now = NanoTimeStamp::now();
        self > &now
    }

    /// Returns the amount of time remaining until the deadline
    /// Returns 0 if the deadline has passed
    pub fn time_until(&self) -> u64 {
        let now = NanoTimeStamp::now();
        if self > &now {
            self.0 - now.0
        } else {
            0
        }
    }

    /// Returns the amount of time that has passed since the deadline
    /// Returns 0 if the deadline is still in the future
    pub fn time_since(&self) -> u64 {
        let now = NanoTimeStamp::now();
        if self < &now {
            now.0 - self.0
        } else {
            0
        }
    }

    /// check if the rate limit has been exceeded
    /// returns true if the rate limit has been exceeded
    pub fn rate_limit_exceeded(&self, rate_limit: u64) -> bool {
        let now = NanoTimeStamp::now();
        let time_since = now.0 - self.0;
        time_since < rate_limit
    }

    /// add seconds to the timestamp
    /// returns a new timestamp
    pub fn add_secs(&self, secs: u64) -> Self {
        let ns_to_add = secs * Self::NS_PER_SECOND;
        NanoTimeStamp(self.0 + ns_to_add)
    }

    /// add minutes to the timestamp
    /// returns a new timestamp
    pub fn add_mins(&self, mins: u64) -> Self {
        let ns_to_add = mins * Self::NS_PER_MINUTE;
        NanoTimeStamp(self.0 + ns_to_add)
    }

    /// add hours to the timestamp
    /// returns a new timestamp
    pub fn add_hours(&self, hours: u64) -> Self {
        let ns_to_add = hours * Self::NS_PER_HOUR;
        NanoTimeStamp(self.0 + ns_to_add)
    }

    /// Add days to the timestamp
    pub fn add_days(&self, days: u64) -> Self {
        let ns_to_add = days * Self::NS_PER_DAY;
        NanoTimeStamp(self.0 + ns_to_add)
    }

    /// Get the number of whole seconds represented by the timestamp
    pub fn get_secs(&self) -> u64 {
        self.0 / Self::NS_PER_SECOND
    }

    /// Get the number of whole minutes represented by the timestamp
    pub fn get_mins(&self) -> u64 {
        self.0 / Self::NS_PER_MINUTE
    }

    /// Get the number of whole hours represented by the timestamp
    pub fn get_hours(&self) -> u64 {
        self.0 / Self::NS_PER_HOUR
    }

    /// Get the number of whole days represented by the timestamp
    pub fn get_days(&self) -> u64 {
        self.0 / Self::NS_PER_DAY
    }
}
