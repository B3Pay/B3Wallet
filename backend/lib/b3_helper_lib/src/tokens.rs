use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tokens {
    pub e8s: u64,
}

impl Tokens {
    /// The maximum number of Tokens we can hold on a single account.
    pub const MAX: Self = Tokens { e8s: u64::MAX };
    /// Zero Tokens.
    pub const ZERO: Self = Tokens { e8s: 0 };
    /// How many times can Tokenss be divided
    pub const SUBDIVIDABLE_BY: u64 = 100_000_000;

    /// Constructs an amount of Tokens from the number of 10^-8 Tokens.
    pub const fn from_e8s(e8s: u64) -> Self {
        Self { e8s }
    }

    /// Returns the number of 10^-8 Tokens in this amount.
    pub const fn e8s(&self) -> u64 {
        self.e8s
    }

    /// Returns the number of 10^-8 Tokens in this amount.
    /// This is the same as `e8s` but is more explicit.
    pub const fn amount(&self) -> u64 {
        self.e8s / Self::SUBDIVIDABLE_BY
    }

    /// Returns if this amount is zero.
    pub const fn is_zero(&self) -> bool {
        self.e8s == 0
    }
}

impl Add for Tokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let e8s = self.e8s.checked_add(other.e8s).unwrap_or_else(|| {
            panic!(
                "Add Tokens {} + {} failed because the underlying u64 overflowed",
                self.e8s, other.e8s
            )
        });
        Self { e8s }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Self) {
        let temp = self.clone() + other;
        *self = temp;
    }
}

impl Sub for Tokens {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let e8s = self.e8s.checked_sub(other.e8s).unwrap_or_else(|| {
            panic!(
                "Subtracting Tokens {} - {} failed because the underlying u64 underflowed",
                self.e8s, other.e8s
            )
        });
        Self { e8s }
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Self) {
        let temp = self.clone() - other;
        *self = temp;
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{:08}",
            self.e8s / Tokens::SUBDIVIDABLE_BY,
            self.e8s % Tokens::SUBDIVIDABLE_BY
        )
    }
}
