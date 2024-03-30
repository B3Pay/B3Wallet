use crate::{
    error::OperationError,
    operation::{Operation, OperationTrait},
    processed::{OperationStatus, ProcessedOperation},
    response::Response,
    types::{ConsentMessage, ResponseMap, UserIds},
};
use b3_utils::{api::AppVersion, principal::StoredPrincipal, types::OperationId, NanoTimeStamp};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct PendingOperation {
    pub id: OperationId,
    pub request: Operation,
    pub status: OperationStatus,
    pub responses: ResponseMap,
    pub deadline: NanoTimeStamp,
    pub created_at: NanoTimeStamp,
    pub created_by: StoredPrincipal,
    pub allowed_signers: UserIds,
    pub consent_message: ConsentMessage,
    pub version: AppVersion,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct RequestArgs {
    pub request: Operation,
    pub reason: String,
    pub version: AppVersion,
    pub allowed_signers: UserIds,
    pub deadline: Option<NanoTimeStamp>,
}

impl PendingOperation {
    pub fn new(
        id: OperationId,
        created_by: StoredPrincipal,
        args: RequestArgs,
    ) -> PendingOperation {
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
            status: OperationStatus::Pending,
            deadline,
            created_at: NanoTimeStamp::now(),
            consent_message,
            version: args.version,
        }
    }

    pub async fn execute(self) -> ProcessedOperation {
        let mut proccess = ProcessedOperation::new(&self);

        let match_result = self.request.execute().await;

        match match_result {
            Ok(message) => proccess.succeed(message),
            Err(err) => proccess.fail(OperationError::ExecutionError(err.to_string())),
        }
    }

    pub fn method(&self) -> String {
        self.request.method_name()
    }

    pub fn signers(&self) -> &ResponseMap {
        &self.responses
    }

    pub fn is_allowed(&self, signer_id: &StoredPrincipal) -> bool {
        self.allowed_signers.iter().any(|id| id == signer_id)
    }

    pub fn is_signed(&self, signer_id: &StoredPrincipal) -> bool {
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
            .filter(|(user, response)| self.allowed_signers.contains(user) && response.is_confirm())
            .count();

        confirmed_responses * 2 > total_signers
    }

    pub fn get_error(&self) -> Option<OperationError> {
        if self.is_rejected() {
            return Some(OperationError::RequestRejected);
        }

        if self.is_expired() {
            return Some(OperationError::RequestExpired);
        }

        None
    }

    pub fn response(
        &mut self,
        user: StoredPrincipal,
        response: Response,
    ) -> Result<(), OperationError> {
        if self.is_signed(&user) {
            return Err(OperationError::RequestAlreadySigned(user));
        }

        if !self.is_allowed(&user) {
            return Err(OperationError::UserNotAllowed(user));
        }

        self.responses.insert(user, response);

        Ok(())
    }
}
