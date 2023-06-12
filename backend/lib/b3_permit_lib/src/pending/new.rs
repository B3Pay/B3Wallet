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

    /// Add a response to the request
    pub fn add_response(&mut self, signer_id: SignerId, response: Response) {
        self.responses.insert(signer_id, response); // Assuming `ResponseMap::insert` exists and `Response` can be inserted
        self.check_status();
    }

    /// Check if the request status needs to be updated
    pub fn check_status(&mut self) {
        if NanoTimeStamp::now() > self.deadline {
            self.status = RequestStatus::Expired;
        } else if self.responses.len() == self.role.get_num_signers() {
            // Assuming `Role::get_num_signers()` exists and returns the number of signers
            self.status = RequestStatus::Success;
        }
    }

    /// Get the current status of the request
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
