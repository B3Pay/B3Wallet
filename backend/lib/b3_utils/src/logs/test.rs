#[cfg(test)]
mod test {
    use crate::{
        log,
        logs::{store::export_log, with_log_mut, LogEntry},
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
            cycle: None,
            version: "0",
        });

        buffer.append(LogEntry {
            timestamp: NanoTimeStamp(1),
            counter: 1,
            message: "Hello, world!".to_string(),
            file: "foo.rs",
            line: 2,
            cycle: None,
            version: "0",
        });

        buffer.append(LogEntry {
            timestamp: NanoTimeStamp(2),
            counter: 2,
            cycle: None,
            message: "Hello, world!".to_string(),
            file: "foo.rs",
            line: 3,
            version: "1",
        });

        let entries = buffer.export();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].counter, 1);
        assert_eq!(entries[1].counter, 2);
        assert_eq!(entries[0].message, "Hello, world!");
        assert_eq!(entries[1].message, "Hello, world!");
        assert_eq!(entries[0].version, "0");
        assert_eq!(entries[1].version, "1");
    }

    #[test]
    fn test_log_loop() {
        for i in 0..109 {
            log!("Hello, {}!", i);
        }

        let entries = export_log();

        assert_eq!(entries.len(), 100);
        assert_eq!(entries[0].counter, 10);
        assert_eq!(entries[99].counter, 109);
        assert_eq!(entries[0].message, "Hello, 9!");
        assert_eq!(entries[99].message, "Hello, 108!");
    }

    #[test]
    fn test_log_loop_with_bigger_capacity() {
        for i in 0..109 {
            log!("Hello, {}!", i);
        }

        let entries = export_log();

        assert_eq!(entries.len(), 100);
        assert_eq!(entries[0].counter, 10);
        assert_eq!(entries[99].counter, 109);
        assert_eq!(entries[0].message, "Hello, 9!");
        assert_eq!(entries[99].message, "Hello, 108!");

        with_log_mut(|log| {
            assert_eq!(log.len(), 100);

            log.set_capacity(1000);

            assert_eq!(log.len(), 100);
            assert_eq!(log.max_capacity(), 1000);
        });
        let entries = export_log();

        assert_eq!(entries[99].message, "Hello, 108!");
    }

    #[test]
    fn test_log_loop_with_smaller_capacity() {
        for i in 0..109 {
            log!("Hello, {}!", i);
        }

        let entries = export_log();

        assert_eq!(entries.len(), 100);
        assert_eq!(entries[0].counter, 10);
        assert_eq!(entries[99].counter, 109);
        assert_eq!(entries[0].message, "Hello, 9!");
        assert_eq!(entries[99].message, "Hello, 108!");

        with_log_mut(|log| {
            assert_eq!(log.len(), 100);

            log.set_capacity(10);

            assert_eq!(log.len(), 10);
        });
        let entries = export_log();

        assert_eq!(entries[9].message, "Hello, 108!");
    }
}
