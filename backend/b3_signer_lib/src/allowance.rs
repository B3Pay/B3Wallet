use b3_helper::types::SignerAllowanceArgs;
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

use crate::types::Metadata;

#[derive(CandidType, Deserialize, Clone)]
pub struct SignerAllowance {
    pub created_at: u64,
    pub updated_at: u64,
    pub limit: Option<u8>,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

impl Default for SignerAllowance {
    fn default() -> Self {
        SignerAllowance {
            metadata: HashMap::new(),
            limit: None,
            created_at: 0,
            updated_at: 0,
            expires_at: None,
        }
    }
}

impl From<SignerAllowanceArgs> for SignerAllowance {
    fn from(allowance: SignerAllowanceArgs) -> Self {
        let now = time();

        SignerAllowance {
            metadata: allowance.metadata.clone(),
            limit: allowance.limit,
            created_at: now,
            updated_at: now,
            expires_at: allowance.expires_at,
        }
    }
}

impl SignerAllowance {
    pub fn new(allowance_args: SignerAllowanceArgs) -> Self {
        allowance_args.into()
    }

    pub fn update(&mut self, allowance_args: SignerAllowanceArgs) {
        self.metadata = allowance_args.metadata.clone();
        self.updated_at = time();
        self.expires_at = allowance_args.expires_at;
    }

    pub fn decrease_limit(&mut self) -> Option<u8> {
        if let Some(limit) = self.limit {
            self.limit = Some(limit - 1);

            self.limit
        } else {
            None
        }
    }

    pub fn is_allowed(&self) -> bool {
        !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            None => false,
            Some(expires_at) => expires_at < time(),
        }
    }
}
