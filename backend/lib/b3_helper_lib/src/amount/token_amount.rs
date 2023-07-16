use crate::token::ICPToken;
use candid::Nat;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use super::error::TokenAmountError;

#[derive(CandidType, Deserialize, PartialEq, Eq, Serialize, Copy, Clone, Debug)]
pub struct TokenAmount {
    pub amount: u128,
    pub decimals: u8,
}

impl PartialOrd for TokenAmount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.amount
                .cmp(&other.amount)
                .then_with(|| self.decimals.cmp(&other.decimals)),
        )
    }
}

impl TokenAmount {
    pub fn new(amount: u128, decimals: u8) -> Self {
        Self { amount, decimals }
    }

    pub fn from_tokens(tokens: ICPToken) -> Self {
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
    pub fn as_u64(&self) -> Result<u64, TokenAmountError> {
        if self.decimals > 0 {
            return Err(TokenAmountError::PrecisionLoss);
        }

        match self.amount.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(TokenAmountError::Overflow),
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
    pub fn as_u128(&self) -> Result<u128, TokenAmountError> {
        if self.decimals > 0 {
            return Err(TokenAmountError::PrecisionLoss);
        }

        Ok(self.amount)
    }

    /// Returns the amount to Nat.
    pub fn to_nat(&self) -> Nat {
        Nat::from(self.amount)
    }

    /// Returns the amount to Satoshi.
    pub fn to_satoshi(&self) -> Result<u64, TokenAmountError> {
        if self.decimals != 8 {
            return Err(TokenAmountError::PrecisionLoss);
        }

        match self.amount.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(TokenAmountError::Overflow),
        }
    }

    /// Returns the Tokens representation of this amount.
    /// Throws an error if the decimals are not 8.
    pub fn to_tokens(&self) -> Result<ICPToken, TokenAmountError> {
        self.try_into()
    }
}

impl Add for TokenAmount {
    type Output = Result<Self, TokenAmountError>;

    fn add(self, other: Self) -> Self::Output {
        if self.decimals != other.decimals {
            return Err(TokenAmountError::DifferentDecimals(
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
            .ok_or(TokenAmountError::Overflow)
    }
}

impl Sub for TokenAmount {
    type Output = Result<Self, TokenAmountError>;

    fn sub(self, other: Self) -> Self::Output {
        if self.decimals != other.decimals {
            return Err(TokenAmountError::DifferentDecimals(
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
            .ok_or(TokenAmountError::Underflow)
    }
}

impl Mul for TokenAmount {
    type Output = Result<Self, TokenAmountError>;

    fn mul(self, other: Self) -> Self::Output {
        match self.amount.checked_mul(other.amount) {
            Some(amount) => {
                let decimals = self.decimals.saturating_add(other.decimals);
                Ok(Self { amount, decimals })
            }
            None => Err(TokenAmountError::Overflow),
        }
    }
}

impl Div for TokenAmount {
    type Output = Result<Self, TokenAmountError>;

    fn div(self, other: Self) -> Self::Output {
        if other.amount == 0 {
            return Err(TokenAmountError::DivisionByZero);
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
            None => Err(TokenAmountError::Underflow),
        }
    }
}

impl From<u128> for TokenAmount {
    fn from(amount: u128) -> Self {
        Self {
            amount,
            decimals: 0,
        }
    }
}

impl TryFrom<TokenAmount> for Nat {
    type Error = TokenAmountError;

    fn try_from(amount: TokenAmount) -> Result<Self, Self::Error> {
        if amount.decimals > 0 {
            return Err(TokenAmountError::PrecisionLoss);
        }

        match amount.amount.try_into() {
            Ok(val) => Ok(Nat(val)),
            Err(_) => Err(TokenAmountError::Overflow),
        }
    }
}

impl TryFrom<&TokenAmount> for ICPToken {
    type Error = TokenAmountError;

    fn try_from(amount: &TokenAmount) -> Result<Self, Self::Error> {
        if amount.decimals != ICPToken::DECIMALS {
            return Err(TokenAmountError::DecimalsMismatch);
        }

        match amount.amount.try_into() {
            Ok(val) => Ok(ICPToken::from_e8s(val)),
            Err(_) => Err(TokenAmountError::Overflow),
        }
    }
}

impl From<ICPToken> for TokenAmount {
    fn from(tokens: ICPToken) -> Self {
        Self {
            amount: tokens.e8s as u128,
            decimals: 8,
        }
    }
}

impl fmt::Display for TokenAmount {
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

impl FromStr for TokenAmount {
    type Err = TokenAmountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        let amount: u128;
        let mut decimals: u8 = 0;

        if parts.len() == 1 {
            // If there's no decimal point
            amount = parts[0]
                .parse::<u128>()
                .map_err(|e| TokenAmountError::InvalidAmount(e.to_string()))?;
        } else if parts.len() == 2 {
            // If there's a decimal point
            decimals = parts[1].len() as u8;
            let whole = parts.join("");
            amount = whole
                .parse::<u128>()
                .map_err(|e| TokenAmountError::InvalidAmount(e.to_string()))?;
        } else {
            return Err(TokenAmountError::ToManyDecimals);
        }

        Ok(Self { amount, decimals })
    }
}
