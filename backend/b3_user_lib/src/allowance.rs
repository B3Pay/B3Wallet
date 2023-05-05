use ic_cdk::api::time;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use std::collections::HashMap;

pub type CanisterId = Principal;
pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Deserialize, Clone)]
pub struct SetAllowance {
    pub metadata: Metadata,
    pub limit: Option<u8>,
    pub expires_at: Option<u64>,
}

pub type Allowances = HashMap<CanisterId, Allowance>;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Allowance {
    pub metadata: Metadata,
    pub limit: Option<u8>,
    pub created_at: u64,
    pub updated_at: u64,
    pub expires_at: Option<u64>,
}

impl Default for Allowance {
    fn default() -> Self {
        Allowance {
            metadata: HashMap::new(),
            limit: None,
            created_at: 0,
            updated_at: 0,
            expires_at: None,
        }
    }
}

impl From<SetAllowance> for Allowance {
    fn from(allowance: SetAllowance) -> Self {
        let now = time();

        Allowance {
            metadata: allowance.metadata.clone(),
            limit: allowance.limit,
            created_at: now,
            updated_at: now,
            expires_at: allowance.expires_at,
        }
    }
}

impl Allowance {
    pub fn new(new_allowance: SetAllowance) -> Self {
        Allowance::from(new_allowance)
    }

    pub fn update(&mut self, new_allowance: SetAllowance) {
        self.metadata = new_allowance.metadata.clone();
        self.updated_at = time();
        self.expires_at = new_allowance.expires_at;
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
