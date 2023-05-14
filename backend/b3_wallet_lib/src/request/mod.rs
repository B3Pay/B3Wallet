pub mod bitcoin;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod message;
pub mod sign;
pub mod state;

use crate::{error::WalletError, signer::Roles, types::RequestId};
use b3_helper::types::SignerId;
use ic_cdk::{
    api::time,
    export::{candid::CandidType, serde::Deserialize},
};
use sign::SignRequest;

#[derive(CandidType, Clone, Deserialize)]
pub struct Request {
    id: RequestId,
    role: Roles,
    deadline: u64,
    request: SignRequest,
    signers: Vec<SignerId>,
}

pub struct RequestArgs {
    pub id: RequestId,
    pub allowed_role: Roles,
    pub request: SignRequest,
    pub deadline: Option<u64>,
}

impl From<RequestArgs> for Request {
    fn from(args: RequestArgs) -> Self {
        let deadline = args.deadline.unwrap_or(time() + 15 * 60 * 1_000_000_000);

        Self {
            deadline,
            id: args.id,
            signers: vec![],
            request: args.request,
            role: args.allowed_role,
        }
    }
}

impl Request {
    pub fn id(&self) -> RequestId {
        self.id
    }

    pub fn role(&self) -> Roles {
        self.role
    }

    pub fn request(&self) -> &SignRequest {
        &self.request
    }

    pub fn deadline(&self) -> u64 {
        self.deadline
    }

    pub fn signers(&self) -> &Vec<SignerId> {
        &self.signers
    }

    pub fn is_expired(&self) -> bool {
        self.deadline < time()
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.signers.contains(signer_id)
    }

    pub fn request_mut(&mut self) -> &mut SignRequest {
        &mut self.request
    }

    pub fn sign(&mut self, signer: SignerId) -> Result<usize, WalletError> {
        if self.signers.contains(&signer) {
            return Err(WalletError::AlreadySigned(signer.to_string()));
        }

        self.signers.push(signer);

        Ok(self.signers.len())
    }
}
