use ic_cdk::api::time;

use crate::types::State;
use b3_shared::types::{Signer, SignerId, UserId};

mod query;
mod update;
mod utils;

impl State {
    pub fn init_signer(&mut self, user: UserId) -> Result<Signer, String> {
        let signer_id = self.get_signer_id(&user);

        if let Some(id) = signer_id {
            return Err(format!("User control already exists: {}!", id));
        }

        let now = time();

        let signer = Signer {
            owner: user,
            signer_id: None,
            created_at: now,
            updated_at: now,
        };

        self.signers.insert(user, signer.clone());

        Ok(signer)
    }

    pub fn add_signer(&mut self, user: UserId, signer_id: SignerId) -> Signer {
        let now = time();

        let signer = self.signers.get(&user).unwrap();

        let finalized_signer = Signer {
            owner: signer.owner,
            signer_id: Some(signer_id),
            created_at: signer.created_at,
            updated_at: now,
        };

        self.signers.insert(user, finalized_signer.clone());

        finalized_signer
    }

    pub fn get_signer(&self, user: &UserId) -> Option<Signer> {
        self.signers.get(user).cloned()
    }

    pub fn get_signer_id(&self, user: &UserId) -> Option<SignerId> {
        let signer = self.signers.get(user).cloned();

        match signer {
            Some(signer) => signer.signer_id,
            None => None,
        }
    }

    pub fn get_user_ids(&self) -> Vec<UserId> {
        self.signers.keys().cloned().collect()
    }

    pub fn remove_signer(&mut self, user: &UserId) {
        self.signers.remove(user);
    }
}
