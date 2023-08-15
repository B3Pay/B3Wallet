mod base32;
mod hasher;
mod ic;

pub(crate) use base32::*;
pub use hasher::*;
pub use ic::*;

#[macro_export]
macro_rules! require {
    ($condition:expr, $($msg:tt)*) => {
        if !$condition {
            return Err(format!($($msg)*));
        }
    };
}
