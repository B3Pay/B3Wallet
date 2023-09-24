mod encoder;
mod ic;

use std::fmt;

pub use encoder::*;
pub use ic::*;

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

#[cfg(feature = "logging")]
pub fn report_log<T, E: fmt::Display>(err: E) -> Result<T, String> {
    crate::log!("{}", err);

    Err(format!("{}", err))
}

#[cfg(feature = "logging")]
pub fn panic_log<T, E: fmt::Display>(err: E) -> T {
    crate::log!("{}", err);

    panic!("{}", err);
}
