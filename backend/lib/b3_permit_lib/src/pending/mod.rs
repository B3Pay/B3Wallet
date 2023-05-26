pub mod btc;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod state;

use crate::confirmed::ConfirmedRequest;
use crate::error::RequestError;
use crate::signer::Roles;
use crate::types::ConsentMessageResponse;
use b3_helper_lib::types::{RequestId, SignerId};
use b3_helper_lib::{error::TrapError, types::Deadline};
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use crate::mocks::ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

use btc::BtcRequest;
use evm::EvmRequest;
use icp::IcpRequest;
use inner::InnerRequest;

#[enum_dispatch(Request)]
pub trait RequestTrait {}

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Request {
    EvmRequest,
    BtcRequest,
    IcpRequest,
    InnerRequest,
}

impl Request {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        match self {
            Request::EvmRequest(args) => args.execute().await,
            Request::BtcRequest(args) => args.execute().await,
            Request::IcpRequest(args) => args.execute().await,
            Request::InnerRequest(args) => args.execute().await,
        }
    }
}

pub struct RequestArgs {
    role: Roles,
    request: Request,
    deadline: Option<Deadline>,
}

impl RequestArgs {
    pub fn new(role: Roles, request: Request, deadline: Option<Deadline>) -> Self {
        RequestArgs {
            role,
            request,
            deadline,
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct PendingRequest {
    id: RequestId,
    role: Roles,
    request: Request,
    deadline: Deadline,
    signers: Vec<SignerId>,
}

impl PendingRequest {
    pub fn new(id: RequestId, args: RequestArgs) -> Self {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            ic_timestamp() + 15 * 60 * 1_000_000_000
        };

        PendingRequest {
            id,
            deadline,
            signers: vec![],
            role: args.role,
            request: args.request,
        }
    }

    pub async fn execute(&self) -> ConfirmedRequest {
        let mut confirmed = ConfirmedRequest::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => confirmed.confirm(message),
            Err(err) => confirmed.reject(RequestError::ExecutionError(err.to_string())),
        }
    }

    pub fn id(&self) -> RequestId {
        self.id
    }

    pub fn role(&self) -> Roles {
        self.role
    }

    pub fn deadline(&self) -> Deadline {
        self.deadline
    }

    pub fn signers(&self) -> &Vec<SignerId> {
        &self.signers
    }

    pub fn is_expired(&self) -> bool {
        self.deadline < ic_timestamp()
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.signers.contains(signer_id)
    }

    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn sign(&mut self, signer: SignerId) -> Result<usize, RequestError> {
        if self.signers.contains(&signer) {
            return Err(RequestError::AlreadySigned(signer.to_string()));
        }

        self.signers.push(signer);

        Ok(self.signers.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::pending::inner::account::RenameAccountRequest;

    use super::*;

    #[test]
    fn test_request_args() {
        let request = RenameAccountRequest {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let args = RequestArgs::new(Roles::Admin, request.into(), None);

        let pending = PendingRequest::new(1, args);

        assert_eq!(pending.id(), 1);
        assert_eq!(pending.role(), Roles::Admin);
        assert_eq!(pending.deadline(), ic_timestamp() + 15 * 60 * 1_000_000_000);
        assert_eq!(pending.is_expired(), false);
    }

    #[test]
    fn test_request_args_with_deadline() {
        let request = RenameAccountRequest {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let args = RequestArgs::new(Roles::Admin, request.into(), Some(1_000_000_000));

        let pending = PendingRequest::new(1, args);

        assert_eq!(pending.id(), 1);
        assert_eq!(pending.role(), Roles::Admin);
        assert_eq!(pending.deadline(), 1_000_000_000);
        assert_eq!(pending.is_expired(), true);
    }
}
