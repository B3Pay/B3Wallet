#[cfg(test)]
mod tests {
    use crate::NanoTimeStamp;

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
        assert_eq!(ts.get_secs(), 1);

        let ts = ts.add_mins(1);
        assert_eq!(ts.to_secs(), 61);
        assert_eq!(ts.get_mins(), 1);

        let ts = ts.add_hours(1);
        assert_eq!(ts.to_secs(), 3661);
        assert_eq!(ts.get_hours(), 1);

        let ts = ts.add_days(1);
        assert_eq!(ts.to_secs(), 90061);
        assert_eq!(ts.get_days(), 1);
    }

    #[test]
    fn test_time_until_and_since() {
        let now = NanoTimeStamp::now();
        let future = now.add_secs(10);

        assert!(future.in_future());
        assert!(!future.has_passed());
        assert!(!now.in_future());

        let time_until = future.time_until();
        assert!(time_until <= 10 * NanoTimeStamp::NS_PER_SECOND);

        let time_since = now.time_since();
        assert!(time_since <= 10 * NanoTimeStamp::NS_PER_SECOND);
    }
}
