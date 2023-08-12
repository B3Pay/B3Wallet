use std::cell::Cell;

use crate::nonce::Nonce;

thread_local! {
    static ENTRY_COUNTER: Cell<Nonce> = Default::default();
}

pub fn increment() -> u64 {
    ENTRY_COUNTER.with(|cell| {
        let mut nonce = cell.take();
        nonce.increment();
        cell.set(nonce);

        nonce.current()
    })
}
