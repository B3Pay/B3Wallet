use std::cell::Cell;

use crate::nonce::Nonce;

thread_local! {
    static LOG_ENTRY_COUNTER: Cell<Nonce> = Default::default();
}

pub fn log_increment() -> u64 {
    LOG_ENTRY_COUNTER.with(|cell| {
        let mut nonce = cell.take();
        nonce.increment();
        cell.set(nonce);

        nonce.get()
    })
}
