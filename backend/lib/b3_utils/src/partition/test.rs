#[cfg(test)]
mod tests {
    use crate::partition::PartitionManager;

    #[test]
    fn test_partition_manager() {
        let mut partition_manager = PartitionManager::init();

        let partition = "test_partition";
        partition_manager.create_partition(partition, 13).unwrap();

        let id = partition_manager.get_partition(partition).unwrap();

        assert_eq!(id, 13);

        let partition = "test_partition2";
        partition_manager.create_partition(partition, 14).unwrap();

        let id = partition_manager.get_partition(partition).unwrap();

        assert_eq!(id, 14);
    }

    #[test]
    fn test_partition_loop() {
        // find duplicates
        let mut partition_manager = PartitionManager::init();

        #[rustfmt::skip]
        let words = [
            "users", "operations", "data", "ledger", "network", "security", "protocol", "interface", 
            "server", "client", "database", "algorithm", "application", "architecture", "array", 
            "upgrade", "bandwidth", "binary", "bit", "buffer", "byte", "cache", "command", "compiler", 
            "compression", "connection", "cookie", "core", "debug", "decode", "disk", "domain", 
            "driver", "encode", "encryption", "firewall", "flag", "folder", "format", "framework", 
            "function", "gateway", "graphic", "hardware", "hash", "heap", "host", "hyperlink", 
            "icon", "index", "input", "internet", "kernel", "keyword", "link", "login", "loop", 
            "malware", "memory", "method", "module", "monitor", "mouse", "network1", "node", "object", 
            "output", "packet", "parameter", "password", "path", "pixel", "platform", "plugin", 
            "pointer", "process", "protocol1", "query", "queue", "recursive", "registry", "root", 
            "router", "runtime", "script", "search", "security1", "server1", "session", "shell", 
            "socket", "software", "spam", "stack", "statement", "string", "syntax", "tag", "thread", 
            "token", "transaction", "trigger", "trojan", "variable", "vector", "virus", "web", "widget", 
            "window", "wireless", "abstract", "assert", "boolean", "break", "bytecode", "callback", 
            "class", "constant", "constructor", "destructor", "exception", "final", "float", "garbage", 
            "generic", "identifier", "implements", "inheritance", "instance", "integer", "interface1", 
            "literal", "native", "null", "operator", "override", "package", "private", "protected", 
            "public", "reference", "reflection", "return", "scope", "static", "super", "synchronized", 
            "this", "throw", "transient", "try", "void", "volatile", "while", "adapter", "agile", 
            "ajax", "applet", "bug", "build", "char", "cloud", "cluster", "code", "commit", "compiler1", 
            "component", "css", "csv", "daemon", "ddl", "debug1", "diagram", "dialog", "div", "dom", 
            "dtd", "entity", "enum", "event", "exception1", "fifo", "file", "source", "setter", 
            "ftp", "getter", "git", "handler", "html", "http", "ide", "template", "int", "jar", "java", 
            "javascript", "jquery", "json", "jsp", "jvm", "key", "lambda", "layer", "library", 
            "linux", "list", "localhost", "let", "long", "const", "machine", "map", "maven", "message", 
            "method1", "mvc", "mysql", "ssh", "nosql", "null1", "object1", "oracle", "pipe", "ssl", 
            "procedure", "process1", "programming", "soap", "sql", "react", "recursion", "refactor", 
            "regex", "repository", "request", "response", "rest", "route", "spring", "runtime1", 
            "scalability", "schema", "script1", "sdk", "servlet", "session1", "soap1", "software1",
            "source1", "sql1", "sql2", "stack1", "string1", "syntax1"
        ];

        for (i, word) in words.iter().enumerate() {
            partition_manager
                .create_partition(&word, (i) as u8)
                .unwrap();

            let id = partition_manager.get_partition(word).unwrap();

            assert_eq!(id, (i) as u8);
        }

        println!("Partitions: {}", partition_manager.partitions.len())
    }

    #[test]
    fn test_stable_vec() {
        let mut partition_manager = PartitionManager::init();
        let partition = "test_partition";
        let vec = partition_manager
            .init_stable_vec::<u32>(partition, 10)
            .unwrap();

        vec.push(&1).unwrap();
        vec.push(&2).unwrap();
        vec.push(&3).unwrap();

        assert_eq!(vec.len(), 3);

        assert_eq!(vec.get(0), Some(1));
        assert_eq!(vec.get(1), Some(2));
        assert_eq!(vec.get(2), Some(3));
    }

    #[test]
    fn test_stable_map() {
        let mut partition_manager = PartitionManager::init();
        let mut map = partition_manager
            .init_stable_map::<u32, u32>("test", 13)
            .unwrap();

        map.insert(1, 1);
        map.insert(2, 2);
        map.insert(3, 3);

        assert_eq!(map.len(), 3);

        assert_eq!(map.get(&1), Some(1));
        assert_eq!(map.get(&2), Some(2));
        assert_eq!(map.get(&3), Some(3));
    }

    #[test]
    fn test_stable_heap() {
        let mut partition_manager = PartitionManager::init();
        let partition = "test_partition";
        let mut heap = partition_manager
            .init_stable_heap::<u32>(partition, 10)
            .unwrap();

        heap.push(&1).unwrap();
        heap.push(&2).unwrap();
        heap.push(&3).unwrap();

        assert_eq!(heap.len(), 3);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
    }

    #[test]
    fn test_stable_log() {
        let mut partition_manager = PartitionManager::init();

        let partition = "test_partition";
        let log = partition_manager
            .init_stable_log::<u32>(partition, 10, 11)
            .unwrap();

        log.append(&1).unwrap();
        log.append(&2).unwrap();
        log.append(&3).unwrap();

        assert_eq!(log.len(), 3);

        assert_eq!(log.get(0), Some(1));
        assert_eq!(log.get(1), Some(2));
        assert_eq!(log.get(2), Some(3));
    }

    #[test]
    fn test_stable_heap_with_stable_vec() {
        let mut partition_manager = PartitionManager::init();
        let partition = "test_partition";
        let partition1 = "test_partition1";
        let mut heap = partition_manager
            .init_stable_heap::<u32>(partition, 10)
            .unwrap();
        let vec = partition_manager
            .init_stable_vec::<u32>(partition1, 11)
            .unwrap();

        vec.push(&1).unwrap();
        vec.push(&2).unwrap();
        vec.push(&3).unwrap();
        heap.push(&1).unwrap();
        heap.push(&2).unwrap();
        heap.push(&3).unwrap();

        assert_eq!(heap.len(), 3);
        assert_eq!(heap.peek(), Some(&1).copied());
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
    }
}
