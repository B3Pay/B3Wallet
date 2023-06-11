use crate::{
    error::RequestError,
    processed::{ProcessedRequest, RequestStatus},
    request::request::{Request, RequestTrait},
    signer::Roles,
    types::{ConsentMessage, Response, ResponseMap},
};
use b3_helper_lib::{
    time::NanoTimeStamp,
    types::{RequestId, SignerId, Version},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct PendingRequest {
    pub id: RequestId,
    pub role: Roles,
    pub request: Request,
    pub status: RequestStatus,
    pub responses: ResponseMap,
    pub deadline: NanoTimeStamp,
    pub created_at: NanoTimeStamp,
    pub consent_message: ConsentMessage,
    pub version: Version,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct RequestArgs {
    pub role: Roles,
    pub request: Request,
    pub reason: String,
    pub version: Version,
    pub deadline: Option<NanoTimeStamp>,
}

impl PendingRequest {
    // Constructor function

    pub fn new(id: RequestId, args: RequestArgs) -> Self {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            NanoTimeStamp::days_from_now(7)
        };

        let consent_message = ConsentMessage::new(&args.request, args.reason);

        PendingRequest {
            responses: ResponseMap::new(),
            request: args.request,
            role: args.role,
            id,
            status: RequestStatus::Pending,
            deadline,
            created_at: NanoTimeStamp::now(),
            consent_message,
            version: args.version,
        }
    }

    // Function to add a response to the request
    pub fn add_response(&mut self, signer_id: SignerId, response: Response) {
        self.responses.insert(signer_id, response); // Assuming `ResponseMap::insert` exists and `Response` can be inserted
        self.check_status();
    }

    // Function to check if the request status needs to be updated
    pub fn check_status(&mut self) {
        if NanoTimeStamp::now() > self.deadline {
            self.status = RequestStatus::Expired;
        } else if self.responses.len() == self.role.get_num_signers() {
            // Assuming `Role::get_num_signers()` exists and returns the number of signers
            self.status = RequestStatus::Success;
        }
    }

    // Function to get the current status of the request
    pub fn get_status(&self) -> RequestStatus {
        self.status.clone() // Assuming `RequestStatus` implements `Clone`
    }

    pub async fn execute(self) -> ProcessedRequest {
        let mut confirmed = ProcessedRequest::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => confirmed.succeed(message),
            Err(err) => confirmed.fail(RequestError::ExecutionError(err.to_string())),
        }
    }

    pub fn method(&self) -> String {
        self.request.method_name()
    }

    pub fn signers(&self) -> &ResponseMap {
        &self.responses
    }

    pub fn is_expired(&self) -> bool {
        self.deadline.has_passed()
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.responses.keys().any(|id| id == signer_id)
    }

    pub fn is_rejected(&self) -> bool {
        self.responses.values().any(|response| response.is_reject())
    }

    pub fn get_error(&self) -> Option<String> {
        if self.is_rejected() {
            return Some(RequestError::RequestRejected.to_string());
        }

        if self.is_expired() {
            return Some(RequestError::RequestExpired.to_string());
        }

        None
    }

    pub fn response(&mut self, signer: SignerId, response: Response) -> Result<(), RequestError> {
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

        let args = RequestArgs {
            role: Roles::Admin,
            request: request.into(),
            reason: "test".to_string(),
            version: "1.0.0".to_string(),
            deadline: None,
        };

        let pending = PendingRequest::new(1, args);

        assert_eq!(pending.id, 1);
        assert_eq!(pending.role, Roles::Admin);
        assert_eq!(pending.deadline, NanoTimeStamp::days_from_now(7));
        assert_eq!(pending.is_expired(), false);
    }

    #[test]
    fn test_request_args_with_deadline() {
        let request = RenameAccount {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let args = RequestArgs {
            role: Roles::Admin,
            request: request.into(),
            reason: "test".to_string(),
            version: "1.0.0".to_string(),
            deadline: Some(NanoTimeStamp(1_000_000_000)),
        };

        let pending = PendingRequest::new(1, args);

        assert_eq!(pending.id, 1);
        assert_eq!(pending.role, Roles::Admin);
        assert_eq!(pending.deadline, NanoTimeStamp(1_000_000_000));
        assert_eq!(pending.is_expired(), true);
    }

    #[test]
    fn test_confirm_request() {
        let request = RenameAccount {
            account_id: "test".to_string(),
            new_name: "test".to_string(),
        };

        let signer = Principal::anonymous();

        let args = RequestArgs {
            role: Roles::Admin,
            request: request.into(),
            reason: "test".to_string(),
            version: "1.0.0".to_string(),
            deadline: None,
        };

        let mut pending = PendingRequest::new(1, args);

        pending.response(signer, Response::Confirm).unwrap();

        assert_eq!(pending.id, 1);
        assert_eq!(pending.role, Roles::Admin);
        assert_eq!(pending.deadline, NanoTimeStamp::days_from_now(7));
        assert_eq!(pending.is_expired(), false);
        assert_eq!(pending.is_signed(&signer), true);
    }

    #[test]
    fn test_response() {
        let response: Response = Response::Confirm;

        if response.is_reject() {
            println!("Response is Reject");
        } else if response.is_confirm() {
            println!("Response is Confirm");
        } else {
            println!("Unknown response");
        }
    }
}
