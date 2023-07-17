use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum TokenAmountError {
    Overflow,
    Underflow,
    PrecisionLoss,
    DivisionByZero,
    ToManyDecimals,
    DecimalsMismatch,
    InvalidAmount(String),
    DifferentDecimals(u8, u8),
}

#[rustfmt::skip]
impl fmt::Display for TokenAmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenAmountError::Overflow => write!(f, "Overflow when adding Amounts"),
            TokenAmountError::Underflow => write!(f, "Underflow when subtracting Amounts"),
            TokenAmountError::PrecisionLoss => write!(f, "Loss of precision when converting to integer"),
            TokenAmountError::DivisionByZero => write!(f, "Division by zero or underflow when dividing Amounts"),
            TokenAmountError::ToManyDecimals => write!(f, "Amount has to many decimals"),
            TokenAmountError::DecimalsMismatch => write!(f, "Amount has wrong number of decimals"),
            TokenAmountError::InvalidAmount(s) => write!(f, "Invalid amount: {}", s),
            TokenAmountError::DifferentDecimals(d1, d2) => write!(f, "Cannot add Amounts with different decimals: {} and {}", d1, d2),
        }
    }
}
