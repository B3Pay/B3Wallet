use ic_cdk::export::Principal;
use std::fmt::Display;
use std::mem::size_of;

use crate::types::{Environment, Subaccount};

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0u8; 32])
    }
}

impl Subaccount {
    pub fn new(environment: Environment, nonce: u64) -> Self {
        let mut subaccount = [0; 32];

        match environment {
            Environment::Production => subaccount[0] = 32,
            Environment::Staging => subaccount[0] = 16,
            Environment::Development => subaccount[0] = 8,
        }

        if nonce >= 255 {
            let count = ((nonce - 1) / 255) as usize;
            subaccount[1..=count].fill(255);
            subaccount[count + 1] = (nonce - (count as u64) * 255 - 1) as u8;
        } else {
            subaccount[1] = nonce as u8;
        }

        Subaccount(subaccount)
    }
}

impl From<Principal> for Subaccount {
    fn from(principal: Principal) -> Self {
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
