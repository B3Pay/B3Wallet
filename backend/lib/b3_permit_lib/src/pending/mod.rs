pub mod btc;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod state;

use std::fmt;

use crate::error::RequestError;
use crate::processed::ProcessedRequest;
use crate::signer::Roles;
use crate::types::{
    ConsentMessageRequest, ConsentMessageResponse, RequestResponse, RequestResponseTrait, Response,
};
use b3_helper_lib::types::{RequestId, SignerId};
use b3_helper_lib::{error::TrapError, types::Deadline};
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use b3_helper_lib::mocks::ic_timestamp;
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

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::EvmRequest(method) => write!(f, "{}", method),
            Request::BtcRequest(method) => write!(f, "{}", method),
            Request::IcpRequest(method) => write!(f, "{}", method),
            Request::InnerRequest(method) => write!(f, "{}", method),
        }
    }
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

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct RequestArgs {
    pub role: Roles,
    pub request: Request,
    pub deadline: Option<Deadline>,
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
    response: Response,
    consent_message: ConsentMessageRequest,
}

impl PendingRequest {
    pub fn new(id: RequestId, args: RequestArgs) -> Self {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            ic_timestamp() + 15 * 60 * 1_000_000_000
        };

        PendingRequest {
            consent_message: ConsentMessageRequest::from(&args),
            response: Response::new(),
            request: args.request,
            role: args.role,
            deadline,
            id,
        }
    }

    pub async fn execute(&self) -> ProcessedRequest {
        let mut confirmed = ProcessedRequest::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => confirmed.succeed(message),
            Err(err) => confirmed.fail(RequestError::ExecutionError(err.to_string())),
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

    pub fn method(&self) -> String {
        self.request.to_string()
    }

    pub fn args(&self) -> RequestArgs {
        RequestArgs {
            role: self.role.clone(),
            request: self.request.clone(),
            deadline: Some(self.deadline),
        }
    }

    pub fn signers(&self) -> &Response {
        &self.response
    }

    pub fn is_expired(&self) -> bool {
        self.deadline < ic_timestamp()
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.response.keys().any(|id| id == signer_id)
    }

    pub fn is_rejected(&self) -> bool {
        self.response.values().any(|response| response.is_reject())
    }

    pub fn get_error(&self) -> Option<RequestError> {
        if self.is_rejected() {
            return Some(RequestError::RequestRejected);
        }

        if self.is_expired() {
            return Some(RequestError::RequestExpired);
        }

        None
    }

    pub fn response(
        &mut self,
        signer: SignerId,
        response: RequestResponse,
    ) -> Result<(), RequestError> {
        if self.is_signed(&signer) {
            return Err(RequestError::RequestAlreadySigned(signer.to_string()));
        }

        self.response.insert(signer, response);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::{pending::inner::account::RenameAccountRequest, types::Confirm};

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

    #[test]
    fn test_confirm_request() {
        let request = RenameAccountRequest {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let signer = Principal::anonymous();

        let args = RequestArgs::new(Roles::Admin, request.into(), None);

        let mut pending = PendingRequest::new(1, args);

        pending
            .response(signer, RequestResponse::Confirm(Confirm))
            .unwrap();

        assert_eq!(pending.id(), 1);
        assert_eq!(pending.role(), Roles::Admin);
        assert_eq!(pending.deadline(), ic_timestamp() + 15 * 60 * 1_000_000_000);
        assert_eq!(pending.is_expired(), false);
        assert_eq!(pending.is_signed(&signer), true);
    }

    #[test]
    fn test_response() {
        let response: RequestResponse = RequestResponse::Confirm(Confirm);

        if response.is_reject() {
            println!("Response is Reject");
        } else if response.is_confirm() {
            println!("Response is Confirm");
        } else {
            println!("Unknown response");
        }
    }
}
