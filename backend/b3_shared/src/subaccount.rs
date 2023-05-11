use ic_cdk::export::Principal;
use std::fmt::Display;
use std::mem::size_of;

use crate::types::Subaccount;

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0; 32])
    }
}

impl From<&Principal> for Subaccount {
    fn from(principal: &Principal) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        let principal_id = principal.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
    }
}

impl Display for Subaccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for byte in self.0.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        write!(f, "{}", result)
    }
}
