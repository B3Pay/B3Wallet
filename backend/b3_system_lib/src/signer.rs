use crate::{error::SystemError, types::State};
use b3_shared::types::{Canister, CanisterId, UserId};

impl State {
    pub fn init_signer(&mut self, user: UserId) -> Result<Canister, SystemError> {
        let canister_id = self.get_canister_id(&user);

        if let Some(id) = canister_id {
            return Err(SystemError::SignerAlreadyExists(id.to_string()));
        }

        let signer = Canister::from(user);

        self.signers.insert(user, signer.clone());

        Ok(signer)
    }

    pub fn add_signer(&mut self, user: UserId, signer: Canister) {
        self.signers.insert(user, signer);
    }

    pub fn get_signer(&self, user: &UserId) -> Option<Canister> {
        self.signers.get(user).cloned()
    }

    pub fn get_canister_id(&self, user: &UserId) -> Option<CanisterId> {
        let signer = self.signers.get(user).cloned();

        match signer {
            Some(signer) => signer.canister_id,
            None => None,
        }
    }

    pub fn get_user_ids(&self) -> Vec<UserId> {
        self.signers.keys().cloned().collect()
    }

    pub fn get_number_of_signers(&self) -> usize {
        self.signers.len()
    }

    pub fn remove_signer(&mut self, user: &UserId) {
        self.signers.remove(user);
    }
}
