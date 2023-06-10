pub mod state;

use crate::{
    error::RequestError,
    processed::ProcessedRequest,
    request::{Request, RequestArgs, RequestTrait},
    signer::Roles,
    types::{ConsentMessage, RequestResponse, Responses},
};
use b3_helper_lib::types::Deadline;
use b3_helper_lib::types::{RequestId, SignerId};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use crate::mocks::ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct PendingRequest {
    id: RequestId,
    role: Roles,
    request: Request,
    deadline: Deadline,
    responses: Responses,
    consent_message: ConsentMessage,
}

impl PendingRequest {
    pub fn new(id: RequestId, args: RequestArgs) -> Self {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            ic_timestamp() + 15 * 60 * 1_000_000_000
        };

        PendingRequest {
            consent_message: ConsentMessage::from(&args.request),
            responses: Responses::new(),
            request: args.request,
            role: args.role,
            deadline,
            id,
        }
    }

    pub async fn execute(self) -> ProcessedRequest {
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

    use crate::request::inner::account::RenameAccount;

    use super::*;

    #[test]
    fn test_request_args() {
        let request = RenameAccount {
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
        let request = RenameAccount {
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
        let request = RenameAccount {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let signer = Principal::anonymous();

        let args = RequestArgs::new(Roles::Admin, request.into(), None);

        let mut pending = PendingRequest::new(1, args);

        pending.response(signer, RequestResponse::Confirm).unwrap();

        assert_eq!(pending.id(), 1);
        assert_eq!(pending.role(), Roles::Admin);
        assert_eq!(pending.deadline(), ic_timestamp() + 15 * 60 * 1_000_000_000);
        assert_eq!(pending.is_expired(), false);
        assert_eq!(pending.is_signed(&signer), true);
    }

    #[test]
    fn test_response() {
        let response: RequestResponse = RequestResponse::Confirm;

        if response.is_reject() {
            println!("Response is Reject");
        } else if response.is_confirm() {
            println!("Response is Confirm");
        } else {
            println!("Unknown response");
        }
    }
}
