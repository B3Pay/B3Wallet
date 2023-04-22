use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};

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
    pub fn derive_hd_path(&self, index: u8) -> Vec<u8> {
        let mut path = Vec::with_capacity(33);
        path.extend_from_slice(&self.0);
        path.extend_from_slice(&index.to_be_bytes());

        path
    }
}
