mod encoder;
mod ic;

use std::fmt;

pub use encoder::*;
pub use ic::*;

use crate::log;

#[macro_export]
macro_rules! require {
    ($condition:expr, $($msg:tt)*) => {
        if !$condition {
            return Err(format!($($msg)*));
        }
    };
}

pub fn report<T, E: fmt::Display>(err: E) -> Result<T, String> {
    Err(format!("{}", err))
}

pub fn report_log<T, E: fmt::Display>(err: E) -> Result<T, String> {
    log!("{}", err);

    Err(format!("{}", err))
}

pub fn panic_log<T, E: fmt::Display>(err: E) -> T {
    log!("{}", err);

    panic!("{}", err);
}
