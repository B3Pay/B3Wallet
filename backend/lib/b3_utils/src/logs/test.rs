#[cfg(test)]
mod test {
    use crate::{
        log,
        logs::{store::export_log, LogEntry},
    };

    #[test]
    fn test_log() {
        log!("Hello, {}!", "world");
        let entries = export_log();

        println!("{}", entries[0]);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].counter, 1);
        assert_eq!(entries[0].message, "Hello, world!");
    }

    #[test]
    fn test_log_multiple() {
        log!("Hello, {}!", "world");
        log!("Hello, {}!", "world");
        log!("Hello, {}!", "world");
        let entries = export_log();

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].counter, 1);
        assert_eq!(entries[1].counter, 2);
        assert_eq!(entries[2].counter, 3);
        assert_eq!(entries[0].message, "Hello, world!");
        assert_eq!(entries[1].message, "Hello, world!");
        assert_eq!(entries[2].message, "Hello, world!");
    }

    #[test]
    fn test_log_buffer() {
        use crate::logs::LogBuffer;
        use crate::NanoTimeStamp;

        let mut buffer = LogBuffer::with_capacity(2);

        buffer.append(LogEntry {
            timestamp: NanoTimeStamp(0),
            counter: 0,
            message: "Hello, world!".to_string(),
            file: "foo.rs",
            line: 1,
        });

        buffer.append(LogEntry {
            timestamp: NanoTimeStamp(1),
            counter: 1,
            message: "Hello, world!".to_string(),
            file: "foo.rs",
            line: 2,
        });

        buffer.append(LogEntry {
            timestamp: NanoTimeStamp(2),
            counter: 2,
            message: "Hello, world!".to_string(),
            file: "foo.rs",
            line: 3,
        });

        let entries: Vec<_> = buffer.iter().cloned().collect();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].counter, 1);
        assert_eq!(entries[1].counter, 2);
        assert_eq!(entries[0].message, "Hello, world!");
        assert_eq!(entries[1].message, "Hello, world!");
    }

    #[test]
    fn test_log_loop() {
        for i in 0..105 {
            log!("Hello, {}!", i);
        }

        let entries = export_log();

        assert_eq!(entries.len(), 100);
        assert_eq!(entries[0].counter, 6);
        assert_eq!(entries[99].counter, 105);
        assert_eq!(entries[0].message, "Hello, 5!");
        assert_eq!(entries[99].message, "Hello, 104!");
    }
}
