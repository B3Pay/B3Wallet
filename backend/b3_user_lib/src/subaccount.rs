use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};

use crate::config::Environment;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Subaccount(pub [u8; 32]);

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0; 32])
    }
}

impl Subaccount {
    pub fn new(principal_id: &Principal) -> Self {
        let mut subaccount = [0; 32];
        let principal_id = principal_id.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
    }

    /// Derives a hierarchical deterministic (HD) path for the subaccount with the given `index`.
    /// The `index` parameter should be a unique integer identifier for the subaccount.
    pub fn derive_hd_path(&self, env: &Environment, index: u8) -> Vec<u8> {
        let mut path = Vec::with_capacity(34);

        path.extend_from_slice(&self.0);

        if env == &Environment::Production {
            path.extend_from_slice(&index.to_be_bytes());
            path.extend_from_slice(&0_u8.to_be_bytes());
        } else {
            path.extend_from_slice(&0_u8.to_be_bytes());
            path.extend_from_slice(&index.to_be_bytes());
        }

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_subaccount_new() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        let subaaccount = Subaccount::new(&principal);

        assert!(subaaccount.0[0] == 0);
        assert!(subaaccount.0[1..6] == [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_subaccount_derive_hd_path_production() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        let subaaccount = Subaccount::new(&principal);

        let path = subaaccount.derive_hd_path(&Environment::Production, 6);

        assert!(path[0..32] == subaaccount.0);
        assert!(path[32..33] == [6]);
        assert!(path[33..34] == [0]);
    }

    #[test]
    fn test_subaccount_derive_hd_path_development() {
        let principal = Principal::from_text("aaaaa-aa").unwrap();
        let subaaccount = Subaccount::new(&principal);

        let path = subaaccount.derive_hd_path(&Environment::Development, 10);

        assert!(path[0..32] == subaaccount.0);
        assert!(path[32..33] == [0]);
        assert!(path[33..34] == [10]);
    }
}
