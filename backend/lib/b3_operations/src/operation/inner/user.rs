use crate::error::OperationError;
use crate::operation::result::OperationResult;
use crate::operation::OperationTrait;
use crate::store::with_permit;
use crate::store::with_permit_mut;
use crate::user::role::UserRole;
use crate::user::UserState;
use async_trait::async_trait;
use b3_utils::types::{Metadata, UserId};
use b3_wallet_lib::error::WalletError;
use candid::{CandidType, Deserialize};

// ADD SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct AddUser {
    pub name: String,
    pub role: UserRole,
    pub signer_id: UserId,
    pub expires_at: Option<u64>,
    pub threshold: Option<u8>,
}

impl From<&AddUser> for UserState {
    fn from(args: &AddUser) -> Self {
        UserState {
            name: args.name.clone(),
            role: args.role.to_owned(),
            expires_at: args.expires_at,
            metadata: Metadata::default(),
        }
    }
}

#[async_trait]
impl OperationTrait for AddUser {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|state| {
            if state.users.contains_key(&signer_id) {
                return Err(WalletError::SignerAlreadyExists(signer_id.to_string()));
            }

            let user = UserState::from(&self);

            state.users.insert(signer_id, user);

            Ok(self.into())
        })
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.threshold.is_some() {
            return Err(OperationError::InvalidThreshold);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "add_signer".to_string()
    }

    fn title(&self) -> String {
        format!("Add user {}", self.name)
    }

    fn message(&self) -> String {
        format!("Add user {}", self.name)
    }
}

// REMOVE SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct RemoveUser {
    pub signer_id: UserId,
}

#[async_trait]
impl OperationTrait for RemoveUser {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|permit| {
            if !permit.users.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            permit.users.remove(&signer_id);

            Ok(self.into())
        })
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        // check if the user exists
        if !with_permit(|permit| permit.users.contains_key(&self.signer_id)) {
            return Err(OperationError::UserDoesNotExist(self.signer_id.to_string()));
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "remove_signer".to_string()
    }

    fn title(&self) -> String {
        format!("Remove user {}", self.signer_id)
    }

    fn message(&self) -> String {
        format!("Remove user {}", self.signer_id)
    }
}
