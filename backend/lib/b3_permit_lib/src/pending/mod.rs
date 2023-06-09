pub mod btc;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod state;

use btc::transfer::*;
use evm::other::*;
use evm::sign::*;
use evm::transfer::*;
use icp::transfer::*;
use inner::account::*;
use inner::setting::*;
use inner::signer::*;

use crate::{
    error::RequestError,
    processed::ProcessedRequest,
    signer::Roles,
    types::{
        ConsentMessageRequest, ConsentMessageResponse, RequestResponse, RequestResponseTrait,
        Responses,
    },
};
use async_trait::async_trait;
use b3_helper_lib::types::Deadline;
use b3_helper_lib::types::{RequestId, SignerId};
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use crate::mocks::ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

#[async_trait]
#[enum_dispatch]
pub trait RequestTrait {
    fn method_name(&self) -> String;
    fn validate_request(&self) -> Result<(), RequestError>;
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError>;
}

#[enum_dispatch(RequestTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Request {
    // EVM
    EvmTransferEthRequest,
    EvmTransferErc20Request,
    EvmDeployContractRequest,
    EvmSignMessageRequest,
    EvmSignTranscationRequest,
    EvmSignRawTransactionRequest,
    // BTC
    BtcTransferRequest,
    // ICP
    IcpTransferRequest,
    TopUpCanisterRequest,
    // INNER
    AddSignerRequest,
    RemoveSignerRequest,
    CreateAccountRequest,
    RemoveAccountRequest,
    RenameAccountRequest,
    HideAccountRequest,
    UnhideAccountRequest,
    UpgradeCanisterRequest,
    UpdateSignerThresholdRequest,
    UpdateCanisterSettingsRequest,
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
    responses: Responses,
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
            responses: Responses::new(),
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
        self.request.method_name()
    }

    pub fn args(&self) -> RequestArgs {
        RequestArgs {
            role: self.role.clone(),
            request: self.request.clone(),
            deadline: Some(self.deadline),
        }
    }

    pub fn signers(&self) -> &Responses {
        &self.responses
    }

    pub fn is_expired(&self) -> bool {
        self.deadline < ic_timestamp()
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.responses.keys().any(|id| id == signer_id)
    }

    pub fn is_rejected(&self) -> bool {
        self.responses.values().any(|response| response.is_reject())
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

        self.responses.insert(signer, response);

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
