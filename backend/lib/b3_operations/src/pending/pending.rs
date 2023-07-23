use crate::{
    error::OperationError,
    operation::{Operation, OperationTrait},
    processed::{OperationStatus, ProcessedOperation},
    response::Response,
    signer::roles::SignerRoles,
    types::{ConsentMessage, ResponseMap, SignerIds},
};
use b3_utils::{
    timestamp::NanoTimeStamp,
    types::{OperationId, SignerId, WalletVersion},
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct PendingOperation {
    pub id: OperationId,
    pub role: SignerRoles,
    pub request: Operation,
    pub status: OperationStatus,
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
    pub role: SignerRoles,
    pub request: Operation,
    pub reason: String,
    pub version: WalletVersion,
    pub allowed_signers: SignerIds,
    pub deadline: Option<NanoTimeStamp>,
}

impl PendingOperation {
    pub fn new(id: OperationId, created_by: SignerId, args: RequestArgs) -> PendingOperation {
        let deadline = if let Some(deadline) = args.deadline {
            deadline
        } else {
            NanoTimeStamp::days_from_now(7)
        };

        let consent_message = ConsentMessage::new(&args.request, args.reason);

        PendingOperation {
            id,
            created_by,
            responses: ResponseMap::new(),
            allowed_signers: args.allowed_signers,
            request: args.request,
            role: args.role,
            status: OperationStatus::Pending,
            deadline,
            created_at: NanoTimeStamp::now(),
            consent_message,
            version: args.version,
        }
    }

    pub async fn execute(self) -> ProcessedOperation {
        let mut confirmed = ProcessedOperation::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => confirmed.succeed(message),
            Err(err) => confirmed.fail(OperationError::ExecutionError(err.to_string())),
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
            return Some(OperationError::RequestRejected.to_string());
        }

        if self.is_expired() {
            return Some(OperationError::RequestExpired.to_string());
        }

        None
    }

    pub fn response(&mut self, signer: SignerId, response: Response) -> Result<(), OperationError> {
        if self.is_signed(&signer) {
            return Err(OperationError::RequestAlreadySigned(signer.to_string()));
        }

        if !self.is_allowed(&signer) {
            return Err(OperationError::SignerNotAllowed(signer.to_string()));
        }

        self.responses.insert(signer, response);

        Ok(())
    }
}
