use crate::{constants::DEFAULT_SUBACCOUNT, utils::base32_encode_account, Subaccount};
use candid::{CandidType, Principal};
use easy_hasher::easy_hasher;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt, hash, str::FromStr};

#[cfg(test)]
use crate::mocks::id_mock as ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use super::error::ICRCAccountError;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct ICRCAccount {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

impl ICRCAccount {
    pub fn new(owner: Principal, subaccount: Option<Subaccount>) -> Self {
        ICRCAccount { owner, subaccount }
    }

    pub fn from_text(text: &str) -> Result<Self, ICRCAccountError> {
        Self::from_str(text)
    }

    #[inline]
    pub fn effective_subaccount(&self) -> &Subaccount {
        self.subaccount.as_ref().unwrap_or(&DEFAULT_SUBACCOUNT)
    }

    fn compute_checksum(&self) -> Vec<u8> {
        // Create a buffer to hold the principal bytes and the subaccount bytes
        let mut buffer = Vec::with_capacity(29 + 32);

        // Add the owner principal bytes
        buffer.extend_from_slice(&self.owner.as_slice());

        // If subaccount exists, add the subaccount bytes. Otherwise add 32 zeros
        match &self.subaccount {
            Some(subaccount) => buffer.extend_from_slice(&subaccount.to_vec()),
            None => buffer.extend_from_slice(&[0u8; 32]),
        }

        // Compute the CRC32 checksum
        easy_hasher::raw_crc32(buffer).to_vec()
    }

    fn compute_base32_checksum(&self) -> String {
        base32_encode_account(&self.compute_checksum())
    }

    pub fn to_text(&self) -> String {
        self.to_string()
    }

    pub fn subaccount(&self) -> Option<Subaccount> {
        self.subaccount.clone()
    }

    pub fn owner(&self) -> Principal {
        self.owner.clone()
    }
}

impl PartialEq for ICRCAccount {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.effective_subaccount() == other.effective_subaccount()
    }
}

impl Eq for ICRCAccount {}

impl cmp::PartialOrd for ICRCAccount {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for ICRCAccount {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.owner.cmp(&other.owner).then_with(|| {
            self.effective_subaccount()
                .cmp(other.effective_subaccount())
        })
    }
}

impl hash::Hash for ICRCAccount {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.effective_subaccount().hash(state);
    }
}

impl From<Principal> for ICRCAccount {
    fn from(principal: Principal) -> Self {
        ICRCAccount {
            owner: principal,
            subaccount: None,
        }
    }
}

impl From<Subaccount> for ICRCAccount {
    fn from(subaccount: Subaccount) -> Self {
        ICRCAccount {
            owner: ic_cdk_id(),
            subaccount: Some(subaccount),
        }
    }
}

impl fmt::Display for ICRCAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subaccount {
            None => write!(f, "{}", self.owner),
            Some(subaccount) => {
                if subaccount.is_default() {
                    write!(f, "{}", self.owner)
                } else {
                    let checksum = self.compute_base32_checksum();
                    let hex_str = subaccount.to_hex();
                    write!(f, "{}-{}.{}", self.owner, checksum, hex_str)
                }
            }
        }
    }
}

impl FromStr for ICRCAccount {
    type Err = ICRCAccountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.len();

        if n == 0 {
            return Err(ICRCAccountError::Malformed("empty".into()));
        }

        let last_dash = s.rfind('-');
        let dot = s.find('.');

        match last_dash {
            None => {
                return Err(ICRCAccountError::Malformed(
                    "expected at least one dash ('-') character".into(),
                ));
            }
            Some(last_dash) => {
                if let Some(dot) = dot {
                    // There is a subaccount
                    let num_subaccount_digits = n - dot - 1;

                    if num_subaccount_digits > 64 {
                        return Err(ICRCAccountError::Malformed(
                            "the subaccount is too long (expected at most 64 characters)".into(),
                        ));
                    };

                    if dot < last_dash {
                        return Err(ICRCAccountError::Malformed(
                            "the subaccount separator does not follow the checksum separator"
                                .into(),
                        ));
                    };

                    if dot - last_dash - 1 != 7 {
                        return Err(ICRCAccountError::BadChecksum);
                    };

                    // The encoding ends with a dot, the subaccount is empty.
                    if dot == n - 1 {
                        return Err(ICRCAccountError::NotCanonical);
                    };

                    // The first digit after the dot must not be a zero.
                    if s.chars().nth(dot + 1).unwrap() == '0' {
                        return Err(ICRCAccountError::NotCanonical);
                    };

                    let principal_text = &s[..last_dash];
                    let owner = Principal::from_text(principal_text)
                        .map_err(|e| ICRCAccountError::InvalidPrincipal(e.to_string()))?;

                    let hex_str = &s[dot + 1..];

                    // Check that the subaccount is not the default.
                    if hex_str.chars().all(|c| c == '0') {
                        return Err(ICRCAccountError::NotCanonical);
                    };

                    let subaccount = Subaccount::from_hex(&hex_str)
                        .map_err(|e| ICRCAccountError::InvalidSubaccount(e.to_string()))?;

                    // Check that the checksum matches the subaccount.
                    let checksum = &s[last_dash + 1..dot];
                    let expected_checksum = base32_encode_account(
                        &ICRCAccount {
                            owner,
                            subaccount: Some(subaccount.clone()),
                        }
                        .compute_checksum(),
                    );

                    if checksum != expected_checksum {
                        return Err(ICRCAccountError::BadChecksum);
                    };

                    Ok(ICRCAccount {
                        owner,
                        subaccount: Some(subaccount),
                    })
                } else {
                    // There is no subaccount, so it's just a Principal
                    let owner = Principal::from_text(s)
                        .map_err(|e| ICRCAccountError::InvalidPrincipal(e.to_string()))?;
                    Ok(ICRCAccount {
                        owner,
                        subaccount: None,
                    })
                }
            }
        }
    }
}
