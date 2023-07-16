use crate::{
    error::PermitError,
    processed::processed::{ProcessedRequest, RequestStatus},
    request::request::{Request, RequestTrait},
    signer::roles::Roles,
    types::{ConsentMessage, Response, ResponseMap, SignerIds},
};
use b3_helper_lib::{
    timestamp::NanoTimeStamp,
    types::{RequestId, SignerId, WalletVersion},
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
    pub created_by: SignerId,
    pub allowed_signers: SignerIds,
    pub consent_message: ConsentMessage,
    pub version: WalletVersion,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct RequestArgs {
    pub role: Roles,
    pub request: Request,
    pub reason: String,
    pub version: WalletVersion,
    pub allowed_signers: SignerIds,
    pub deadline: Option<NanoTimeStamp>,
}

impl PendingRequest {
    pub fn new(id: RequestId, created_by: SignerId, args: RequestArgs) -> PendingRequest {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            NanoTimeStamp::days_from_now(7)
        };

        let consent_message = ConsentMessage::new(&args.request, args.reason);

        PendingRequest {
            id,
            created_by,
            responses: ResponseMap::new(),
            allowed_signers: args.allowed_signers,
            request: args.request,
            role: args.role,
            status: RequestStatus::Pending,
            deadline,
            created_at: NanoTimeStamp::now(),
            consent_message,
            version: args.version,
        }
    }

    pub async fn execute(self) -> ProcessedRequest {
        let mut confirmed = ProcessedRequest::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => confirmed.succeed(message),
            Err(err) => confirmed.fail(PermitError::ExecutionError(err.to_string())),
        }
    }

    pub fn method(&self) -> String {
        self.request.method_name()
    }

    pub fn signers(&self) -> &ResponseMap {
        &self.responses
    }

    pub fn is_allowed(&self, signer_id: &SignerId) -> bool {
        self.allowed_signers.iter().any(|id| id == signer_id)
    }

    pub fn is_signed(&self, signer_id: &SignerId) -> bool {
        self.responses.keys().any(|id| id == signer_id)
    }

    pub fn is_failed(&self) -> bool {
        self.get_error().is_some()
    }

    pub fn is_expired(&self) -> bool {
        self.deadline.has_passed()
    }

    pub fn is_rejected(&self) -> bool {
        let total_signers = self.allowed_signers.len();
        let rejected_responses = self.responses.values().filter(|r| r.is_reject()).count();

        rejected_responses >= (total_signers + 1) / 2
    }

    pub fn is_confirmed(&self) -> bool {
        let total_signers = self.allowed_signers.len();
        let confirmed_responses = self
            .responses
            .iter()
            .filter(|(signer, response)| {
                self.allowed_signers.contains(signer) && response.is_confirm()
            })
            .count();

        confirmed_responses * 2 > total_signers
    }

    pub fn get_error(&self) -> Option<String> {
        if self.is_rejected() {
            return Some(PermitError::RequestRejected.to_string());
        }

        if self.is_expired() {
            return Some(PermitError::RequestExpired.to_string());
        }

        None
    }

    pub fn response(&mut self, signer: SignerId, response: Response) -> Result<(), PermitError> {
        if self.is_signed(&signer) {
            return Err(PermitError::RequestAlreadySigned(signer.to_string()));
        }

        if !self.is_allowed(&signer) {
            return Err(PermitError::SignerNotAllowed(signer.to_string()));
        }

        self.responses.insert(signer, response);

        Ok(())
    }
}
