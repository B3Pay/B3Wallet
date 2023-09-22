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
