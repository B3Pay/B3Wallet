pub mod bitcoin;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod message;
pub mod sign;

use crate::{error::SignerError, types::RequestId};
use b3_helper::types::SignerId;
use ic_cdk::{
    api::time,
    export::{candid::CandidType, serde::Deserialize},
};
use sign::SignRequest;

#[derive(CandidType, Clone, Deserialize)]
pub struct Request {
    pub id: RequestId,
    pub deadline: u64,
    pub request: SignRequest,
    pub signers: Vec<SignerId>,
}

impl Request {
    pub fn new(id: RequestId, request: SignRequest, deadline: Option<u64>) -> Self {
        let deadline = deadline.unwrap_or(time() + 15 * 60 * 1_000_000_000);

        Self {
            id,
            request,
            deadline,
            signers: Vec::new(),
        }
    }

    pub fn id(&self) -> RequestId {
        self.id
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
    pub fn request_mut(&mut self) -> &mut SignRequest {
        &mut self.request
    }

    pub fn sign(&mut self, signer: SignerId) -> Result<usize, SignerError> {
        if self.signers.contains(&signer) {
            return Err(SignerError::AlreadySigned(signer.to_string()));
        }

        self.signers.push(signer);

        Ok(self.signers.len())
    }
}
