#[cfg(test)]
mod tests {
    use crate::memory::{
        types::{DefaultVMHeap, DefaultVMLog, DefaultVMMap, DefaultVMVec},
        with_stable_mem, with_stable_mem_mut, StableMemoryManager,
    };

    #[test]
    fn test_memory() {
        with_stable_mem(|memory| {
            assert_eq!(memory.partitions.len(), 0);
        });
    }

    #[test]
    fn test_create_partition() {
        with_stable_mem_mut(|memory| {
            let partition1 = memory.create("test", 1);

            assert!(partition1.is_ok());

            let partition2 = memory.create("test", 2);

            assert!(partition2.is_err());

            let partition3 = memory.create("test2", 1);

            assert!(partition3.is_err());
        });
    }

    #[test]
    fn test_partition() {
        let mut memory = StableMemoryManager::init();

        let partition_name = "test_partition";
        memory.create(partition_name, 13).unwrap();

        let id = memory.partition(partition_name).unwrap();

        assert_eq!(id, 13);

        let partition_name = "test_partition2";
        memory.create(partition_name, 14).unwrap();

        let id = memory.partition(partition_name).unwrap();

        assert_eq!(id, 14);
    }

    #[test]
    fn test_partition_loop() {
        let mut stable_memory = StableMemoryManager::init();

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
            stable_memory.create(&word, (i) as u8).unwrap();

            let id = stable_memory.partition(word).unwrap();

            assert_eq!(id, (i) as u8);
        }

        println!("Partitions: {}", stable_memory.partitions.len())
    }

    #[test]
    fn test_stable_vec() {
        let mut stable_memory = StableMemoryManager::init();

        let vec: DefaultVMVec<u32> = stable_memory.init_memory("test_partition", 10).unwrap();

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
        let mut stable_memory = StableMemoryManager::init();

        let mut map: DefaultVMMap<u32, u32> = stable_memory.init_memory("test", 13).unwrap();

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
        let mut stable_memory = StableMemoryManager::init();

        let mut heap: DefaultVMHeap<u32> = stable_memory.init_memory("test_partition", 10).unwrap();

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
        let mut stable_memory = StableMemoryManager::init();

        let log: DefaultVMLog<u32> = stable_memory.init_memory("test_partition", 10).unwrap();

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
        let mut stable_memory = StableMemoryManager::init();

        let mut min_heap: DefaultVMHeap<u32> =
            stable_memory.init_memory("test_partition", 10).unwrap();

        let vec: DefaultVMVec<u32> = stable_memory.init_memory("test_partition1", 11).unwrap();

        vec.push(&1).unwrap();
        vec.push(&2).unwrap();
        vec.push(&3).unwrap();
        min_heap.push(&1).unwrap();
        min_heap.push(&2).unwrap();
        min_heap.push(&3).unwrap();

        assert_eq!(min_heap.len(), 3);
        assert_eq!(min_heap.peek(), Some(&1).copied());
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.pop(), Some(3));
    }

    #[test]
    fn test_init() {
        let stable_memory = StableMemoryManager::init();

        stable_memory.memory_manager();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 0);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);
    }

    #[test]
    fn test_get_or_create() {
        let mut stable_memory = StableMemoryManager::init();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 0);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);

        stable_memory.create("test", 1).unwrap();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 1);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);

        stable_memory.memory("test").unwrap();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 1);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);

        let memory = stable_memory.create("test", 1);

        assert!(memory.is_ok());

        let memory = stable_memory.create("test2", 1);

        assert!(memory.is_err());

        stable_memory.create("test2", 2).unwrap();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 2);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);
    }

    #[test]
    fn test_init_memory() {
        let mut stable_memory = StableMemoryManager::init();

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 0);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);

        let memory = stable_memory
            .init_memory::<DefaultVMVec<u8>>("test", 1)
            .unwrap();

        memory.push(&1).unwrap();

        assert_eq!(memory.len(), 1);

        let partitions = stable_memory.partitions();

        assert_eq!(partitions.len(), 1);

        let backup = stable_memory.backup();

        assert_eq!(backup.len(), 0);
    }
}
