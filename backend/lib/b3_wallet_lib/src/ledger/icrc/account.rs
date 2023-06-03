use b3_helper_lib::{constants::DEFAULT_SUBACCOUNT, types::Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use std::{cmp, fmt, hash};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct IcrcAccount {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

impl IcrcAccount {
    #[inline]
    pub fn effective_subaccount(&self) -> &Subaccount {
        self.subaccount.as_ref().unwrap_or(&DEFAULT_SUBACCOUNT)
    }
}

impl PartialEq for IcrcAccount {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.effective_subaccount() == other.effective_subaccount()
    }
}

impl Eq for IcrcAccount {}

impl cmp::PartialOrd for IcrcAccount {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for IcrcAccount {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.owner.cmp(&other.owner).then_with(|| {
            self.effective_subaccount()
                .cmp(other.effective_subaccount())
        })
    }
}

impl hash::Hash for IcrcAccount {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.effective_subaccount().hash(state);
    }
}

impl fmt::Display for IcrcAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subaccount {
            None => write!(f, "{}", self.owner),
            Some(subaccount) => write!(f, "0x{}.{}", subaccount.to_hex(), self.owner),
        }
    }
}

impl From<Principal> for IcrcAccount {
    fn from(owner: Principal) -> Self {
        Self {
            owner,
            subaccount: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use b3_helper_lib::{constants::DEFAULT_SUBACCOUNT, types::Subaccount};
    use candid::Principal;

    // mock values
    const MOCK_OWNER: Principal = Principal::anonymous();

    #[test]
    fn test_effective_subaccount() {
        let mock_subaccount = Some(Subaccount::default());

        let account1 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: None,
        };

        let account2 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: mock_subaccount.clone(),
        };

        assert_eq!(account1.effective_subaccount(), &DEFAULT_SUBACCOUNT);
        assert_eq!(account2.effective_subaccount(), &mock_subaccount.unwrap());
    }

    #[test]
    fn test_account_equality() {
        let mock_subaccount = Some(Subaccount::default());

        let account1 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: mock_subaccount.clone(),
        };

        let account2 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: mock_subaccount.clone(),
        };

        assert_eq!(account1, account2);
    }

    #[test]
    fn test_account_ordering() {
        let account1 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: None,
        };

        let mock_subaccount = Some(Subaccount([1; 32]));

        let account2 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: mock_subaccount.clone(),
        };

        assert!(account1 < account2);
    }

    #[test]
    fn test_account_display() {
        let mock_subaccount = Some(Subaccount::default());

        let account1 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: None,
        };

        assert_eq!(format!("{}", account1), format!("{}", MOCK_OWNER));

        let account2 = IcrcAccount {
            owner: MOCK_OWNER.clone(),
            subaccount: mock_subaccount.clone(),
        };

        assert_eq!(
            format!("{}", account2),
            format!("0x{}.{}", mock_subaccount.unwrap().to_hex(), MOCK_OWNER)
        );
    }
}
