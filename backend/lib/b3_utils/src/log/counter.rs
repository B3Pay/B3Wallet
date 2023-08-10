use std::cell::Cell;

thread_local! {
    static ENTRY_COUNTER: Cell<u64> = Default::default();
}

pub fn increment() -> u64 {
    ENTRY_COUNTER.with(|cell| {
        cell.set(cell.get() + 1);
        cell.get()
    })
}

pub fn set(value: u64) {
    ENTRY_COUNTER.with(|cell| cell.set(value));
}

pub fn get() -> u64 {
    ENTRY_COUNTER.with(|cell| cell.get())
}
