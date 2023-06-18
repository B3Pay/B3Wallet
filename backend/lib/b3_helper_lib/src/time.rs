use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

#[cfg(test)]
use crate::mocks::ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

#[derive(CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NanoTimeStamp(pub u64);

impl NanoTimeStamp {
    // Constants for nanosecond conversions
    pub const NS_PER_SECOND: u64 = 1_000_000_000;
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

    /// Converts the timestamp to seconds
    pub fn to_secs(&self) -> u64 {
        self.0 / 1_000_000_000
    }

    /// Converts the timestamp to milliseconds
    pub fn to_millis(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Checks if the deadline has passed
    pub fn has_passed(&self) -> bool {
        let now = NanoTimeStamp::now();
        self < &now
    }

    /// Checks if the deadline is still in the future
    pub fn is_future(&self) -> bool {
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

    /// Get the number of whole days represented by the timestamp
    pub fn get_days(&self) -> u64 {
        self.0 / Self::NS_PER_DAY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_conversions() {
        let one_sec = NanoTimeStamp::NS_PER_SECOND;
        let one_min = NanoTimeStamp::NS_PER_MINUTE;
        let one_hour = NanoTimeStamp::NS_PER_HOUR;
        let one_day = NanoTimeStamp::NS_PER_DAY;

        let ts = NanoTimeStamp(one_day + one_hour + one_min + one_sec);
        assert_eq!(ts.to_secs(), 90061);
        assert_eq!(ts.to_millis(), 90061000);
        assert_eq!(ts.get_days(), 1);
    }

    #[test]
    fn test_add_functions() {
        let ts = NanoTimeStamp(0);

        let ts = ts.add_secs(1);
        assert_eq!(ts.to_secs(), 1);

        let ts = ts.add_mins(1);
        assert_eq!(ts.to_secs(), 61);

        let ts = ts.add_hours(1);
        assert_eq!(ts.to_secs(), 3661);

        let ts = ts.add_days(1);
        assert_eq!(ts.get_days(), 1);
    }

    #[test]
    fn test_time_until_and_since() {
        let now = NanoTimeStamp::now();
        let future = now.add_secs(10);
        let past = NanoTimeStamp(now.0 - 10 * NanoTimeStamp::NS_PER_SECOND);

        assert!(future.is_future());
        assert!(!future.has_passed());
        assert!(past.has_passed());
        assert!(!past.is_future());

        let time_until = future.time_until();
        assert!(time_until > 0 && time_until <= 10 * NanoTimeStamp::NS_PER_SECOND);

        let time_since = past.time_since();
        assert!(time_since > 0 && time_since <= 10 * NanoTimeStamp::NS_PER_SECOND);
    }
}