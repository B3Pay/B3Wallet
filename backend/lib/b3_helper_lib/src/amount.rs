use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use candid::Nat;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use crate::tokens::Tokens;

#[derive(CandidType, Deserialize, PartialEq, Eq, Serialize, Copy, Clone, Debug)]
pub struct Amount {
    pub amount: u128,
    pub decimals: u8,
}

impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.amount
                .cmp(&other.amount)
                .then_with(|| self.decimals.cmp(&other.decimals)),
        )
    }
}

impl Amount {
    pub fn new(amount: u128, decimals: u8) -> Self {
        Self { amount, decimals }
    }

    pub fn from_tokens(tokens: Tokens) -> Self {
        Self {
            amount: tokens.e8s as u128,
            decimals: 8,
        }
    }

    /// Returns the amount as a u64, if the amount has no decimals.
    /// Otherwise returns an error.
    /// # Example
    /// ```
    /// use b3_helper_lib::amount::Amount;
    ///
    /// let amount = Amount::new(100, 0);
    ///
    /// assert_eq!(amount.as_u64().unwrap(), 100);
    ///
    /// let amount = Amount::new(100, 1);
    ///
    /// assert!(amount.as_u64().is_err());
    /// ```
    pub fn as_u64(&self) -> Result<u64, AmountError> {
        if self.decimals > 0 {
            return Err(AmountError::PrecisionLoss);
        }

        match self.amount.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(AmountError::Overflow),
        }
    }

    /// Returns the amount as a u128, if the amount has no decimals.
    /// Otherwise returns an error.
    ///
    /// # Example
    /// ```
    /// use b3_helper_lib::amount::Amount;
    ///
    /// let amount = Amount::new(100, 0);
    ///
    /// assert_eq!(amount.as_u128().unwrap(), 100);
    /// ```
    pub fn as_u128(&self) -> Result<u128, AmountError> {
        if self.decimals > 0 {
            return Err(AmountError::PrecisionLoss);
        }

        Ok(self.amount)
    }

    /// Returns the amount to Nat.
    pub fn to_nat(&self) -> Nat {
        Nat::from(self.amount)
    }

    /// Returns the amount to Satoshi.
    pub fn to_satoshi(&self) -> Result<u64, AmountError> {
        if self.decimals != 8 {
            return Err(AmountError::PrecisionLoss);
        }

        match self.amount.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(AmountError::Overflow),
        }
    }

    /// Returns the Tokens representation of this amount.
    /// Throws an error if the decimals are not 8.
    pub fn to_tokens(&self) -> Result<Tokens, AmountError> {
        self.try_into()
    }
}

impl Add for Amount {
    type Output = Result<Self, AmountError>;

    fn add(self, other: Self) -> Self::Output {
        if self.decimals != other.decimals {
            return Err(AmountError::DifferentDecimals(
                self.decimals,
                other.decimals,
            ));
        }

        self.amount
            .checked_add(other.amount)
            .map(|amount| Self {
                amount,
                decimals: self.decimals,
            })
            .ok_or(AmountError::Overflow)
    }
}

impl Sub for Amount {
    type Output = Result<Self, AmountError>;

    fn sub(self, other: Self) -> Self::Output {
        if self.decimals != other.decimals {
            return Err(AmountError::DifferentDecimals(
                self.decimals,
                other.decimals,
            ));
        }

        self.amount
            .checked_sub(other.amount)
            .map(|amount| Self {
                amount,
                decimals: self.decimals,
            })
            .ok_or(AmountError::Underflow)
    }
}

impl Mul for Amount {
    type Output = Result<Self, AmountError>;

    fn mul(self, other: Self) -> Self::Output {
        match self.amount.checked_mul(other.amount) {
            Some(amount) => {
                let decimals = self.decimals.saturating_add(other.decimals);
                Ok(Self { amount, decimals })
            }
            None => Err(AmountError::Overflow),
        }
    }
}

impl Div for Amount {
    type Output = Result<Self, AmountError>;

    fn div(self, other: Self) -> Self::Output {
        if other.amount == 0 {
            return Err(AmountError::DivisionByZero);
        }

        // Adjust the divisor and dividend to have the same decimal places
        let max_decimals = self.decimals.max(other.decimals);
        let self_amount = self.amount * 10u128.pow((max_decimals - self.decimals) as u32);
        let other_amount = other.amount * 10u128.pow((max_decimals - other.decimals) as u32);

        match self_amount.checked_div(other_amount) {
            Some(amount) => Ok(Self {
                amount,
                decimals: max_decimals,
            }),
            None => Err(AmountError::Underflow),
        }
    }
}

impl From<u128> for Amount {
    fn from(amount: u128) -> Self {
        Self {
            amount,
            decimals: 0,
        }
    }
}

impl TryFrom<Amount> for Nat {
    type Error = AmountError;

    fn try_from(amount: Amount) -> Result<Self, Self::Error> {
        if amount.decimals > 0 {
            return Err(AmountError::PrecisionLoss);
        }

        match amount.amount.try_into() {
            Ok(val) => Ok(Nat(val)),
            Err(_) => Err(AmountError::Overflow),
        }
    }
}

impl TryFrom<&Amount> for Tokens {
    type Error = AmountError;

    fn try_from(amount: &Amount) -> Result<Self, Self::Error> {
        if amount.decimals != Tokens::DECIMALS {
            return Err(AmountError::DecimalsMismatch);
        }

        match amount.amount.try_into() {
            Ok(val) => Ok(Tokens::from_e8s(val)),
            Err(_) => Err(AmountError::Overflow),
        }
    }
}

impl From<Tokens> for Amount {
    fn from(tokens: Tokens) -> Self {
        Self {
            amount: tokens.e8s as u128,
            decimals: 8,
        }
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let amount = self.amount.to_string();
        let len = amount.len();

        if self.decimals > 0 && len > self.decimals as usize {
            let (integral, fractional) = amount.split_at(len - self.decimals as usize);
            let fractional = fractional.trim_end_matches('0');
            if fractional.is_empty() {
                write!(f, "{}", integral)
            } else {
                write!(f, "{}.{}", integral, fractional)
            }
        } else {
            if self.decimals == 0 {
                write!(f, "{}", amount)
            } else {
                let zeros = if len <= self.decimals as usize {
                    "0".repeat(self.decimals as usize - len)
                } else {
                    String::new()
                };
                let result = format!("0.{}{}", zeros, amount);
                let result = result.trim_end_matches('0');
                if result.ends_with('.') {
                    write!(f, "{}", result.trim_end_matches('.'))
                } else {
                    write!(f, "{}", result)
                }
            }
        }
    }
}

impl FromStr for Amount {
    type Err = AmountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        let amount: u128;
        let mut decimals: u8 = 0;

        if parts.len() == 1 {
            // If there's no decimal point
            amount = parts[0]
                .parse::<u128>()
                .map_err(|e| AmountError::InvalidAmount(e.to_string()))?;
        } else if parts.len() == 2 {
            // If there's a decimal point
            decimals = parts[1].len() as u8;
            let whole = parts.join("");
            amount = whole
                .parse::<u128>()
                .map_err(|e| AmountError::InvalidAmount(e.to_string()))?;
        } else {
            return Err(AmountError::ToManyDecimals);
        }

        Ok(Self { amount, decimals })
    }
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum AmountError {
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
impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmountError::Overflow => write!(f, "Overflow when adding Amounts"),
            AmountError::Underflow => write!(f, "Underflow when subtracting Amounts"),
            AmountError::PrecisionLoss => write!(f, "Loss of precision when converting to integer"),
            AmountError::DivisionByZero => write!(f, "Division by zero or underflow when dividing Amounts"),
            AmountError::ToManyDecimals => write!(f, "Amount has to many decimals"),
            AmountError::DecimalsMismatch => write!(f, "Amount has wrong number of decimals"),
            AmountError::InvalidAmount(s) => write!(f, "Invalid amount: {}", s),
            AmountError::DifferentDecimals(d1, d2) => write!(f, "Cannot add Amounts with different decimals: {} and {}", d1, d2),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 8);

        assert_eq!(amount1 + amount2, Ok(Amount::new(200, 8)));

        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 0);

        assert_eq!(amount1 + amount2, Err(AmountError::DifferentDecimals(8, 0)));

        let amount1 = Amount::new(u128::MAX, 8);
        let amount2 = Amount::new(1, 8);

        assert_eq!(amount1 + amount2, Err(AmountError::Overflow),);
    }

    #[test]
    fn test_sub() {
        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 8);

        assert_eq!(amount1 - amount2, Ok(Amount::new(0, 8)));

        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 0);

        assert_eq!(amount1 - amount2, Err(AmountError::DifferentDecimals(8, 0)));

        let amount1 = Amount::new(0, 8);
        let amount2 = Amount::new(1, 8);

        assert_eq!(amount1 - amount2, Err(AmountError::Underflow),);
    }

    #[test]
    fn test_mul() {
        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 8);

        assert_eq!(amount1 * amount2, Ok(Amount::new(10000, 16)));
        assert_eq!("0.000000000001", Amount::new(10000, 16).to_string());

        let amount1 = Amount::new(100, 0);
        let amount2 = Amount::new(100, 8);

        assert_eq!(amount1 * amount2, Ok(Amount::new(10000, 8)));

        let amount1 = Amount::new(u128::MAX, 8);
        let amount2 = Amount::new(2, 8);

        assert_eq!(amount1 * amount2, Err(AmountError::Overflow),);
    }

    #[test]
    fn test_div() {
        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(100, 8);

        assert_eq!(amount1 / amount2, Ok(Amount::new(1, 8)));

        let amount1 = Amount::new(100, 18);
        let amount2 = Amount::new(100, 18);

        assert_eq!(amount1 / amount2, Ok(Amount::new(1, 18)));

        let amount1 = Amount::new(10000000000, 8);
        let amount2 = Amount::new(100, 0);

        assert_eq!(amount1 / amount2, Ok(Amount::new(1, 8)));

        let amount1 = Amount::new(100, 8);
        let amount2 = Amount::new(0, 8);

        assert_eq!(amount1 / amount2, Err(AmountError::DivisionByZero),);
    }

    #[test]
    fn test_from_tokens() {
        let amount = Amount::from_tokens(Tokens::from_e8s(1));

        assert_eq!(amount, Amount::new(1, 8));

        let amount = Amount::from_tokens(Tokens::from_e8s(1000000000));

        assert_eq!(amount, Amount::new(1000000000, 8));
    }

    #[test]
    fn test_to_nat() {
        let amount = Amount::new(100, 8);

        assert_eq!(amount.to_nat(), 100);

        let amount = Amount::new(1000000000000000000, 0);

        assert_eq!(amount.to_nat(), Nat::from(1000000000000000000u64));

        let amount = Amount::new(1010000000000000000, 18);

        assert_eq!(amount.to_nat(), Nat::from(1010000000000000000u64));
    }

    #[test]
    fn test_display() {
        let amount = Amount::new(100, 8);

        assert_eq!(amount.to_string(), "0.000001");

        let amount = Amount::new(1000000000000000000, 0);

        assert_eq!(amount.to_string(), "1000000000000000000");

        let amount = Amount::new(1010000000000000000, 18);

        assert_eq!(amount.to_string(), "1.01");

        let amount = Amount::new(1000000001000000000, 18);

        assert_eq!(amount.to_string(), "1.000000001");

        let amount = Amount::new(100, 1);

        assert_eq!(amount.to_string(), "10");

        let amount = Amount::new(100, 2);

        assert_eq!(amount.to_string(), "1");

        let amount = Amount::new(100, 3);

        assert_eq!(amount.to_string(), "0.1");
    }

    #[test]
    fn test_from_str() {
        let amount = "10.00000000".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(1000000000, 8));

        let amount = "1000000000000000000".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(1000000000000000000, 0));

        let amount = "0.000000000000000001".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(1, 18));

        let amount = "100".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(100, 0));

        let amount = "1.0".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(10, 1));

        let amount = "1.001".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(1001, 3));

        let amount = "1.00000001".parse::<Amount>().unwrap();

        assert_eq!(amount, Amount::new(100000001, 8));
    }
}
