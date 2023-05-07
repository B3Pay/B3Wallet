use std::collections::HashMap;

use crate::error::SignerError;
use crate::ledger::config::Environment;
use crate::ledger::ledger::Ledger;
use crate::ledger::public_keys::PublicKeys;
use crate::ledger::subaccount::Subaccount;
use crate::types::CanisterHashMap;
use crate::{request::SignRequest, signed::SignedTransaction, transaction::get_transaction};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::allowance::{Allowance, CanisterId, SetAllowance};

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub ledger: Ledger,
    pub signed: SignedTransaction,
    pub requests: HashMap<CanisterId, SignRequest>,
    pub canisters: HashMap<CanisterId, Allowance>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: String::new(),
            name: String::new(),
            requests: HashMap::new(),
            canisters: HashMap::new(),
            signed: SignedTransaction::default(),
            ledger: Ledger::default(),
        }
    }
}

impl Account {
    pub fn new(subaccount: Subaccount) -> Self {
        let id = subaccount.get_id();
        let ledger = Ledger::new(subaccount);

        Account {
            id,
            ledger,
            name: String::new(),
            requests: HashMap::new(),
            canisters: HashMap::new(),
            signed: SignedTransaction::default(),
        }
    }
    pub async fn request_ecdsa_public_key(&mut self) -> Result<Vec<u8>, SignerError> {
        let ecdsa = self.ledger.ecdsa_public_key().await?;

        self.ledger.public_keys.set_ecdsa(ecdsa)
    }

    pub async fn sign_transaction(
        &self,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> Result<SignedTransaction, SignerError> {
        let ecdsa = self.ledger.public_keys.get_ecdsa()?;

        let mut tx = get_transaction(&hex_raw_tx, chain_id).unwrap();

        let message = tx.get_message_to_sign().unwrap();

        assert!(message.len() == 32);

        let signature = self.ledger.sign_with_ecdsa(message).await?;

        let signed_tx = tx.sign(signature, ecdsa).unwrap();

        Ok(SignedTransaction::new(signed_tx))
    }

    pub fn new_request(
        &mut self,
        from: CanisterId,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> SignRequest {
        let request = SignRequest::new(hex_raw_tx, chain_id, None);

        self.requests.insert(from, request.clone());

        request
    }

    pub fn insert_signed_transaction(&mut self, signed_tx: SignedTransaction) {
        self.signed = signed_tx;
    }

    pub fn insert_canister(&mut self, canister_id: CanisterId, new_allowance: SetAllowance) {
        let allowance = Allowance::new(new_allowance);

        self.canisters.insert(canister_id, allowance);
    }

    pub fn remove_canister(&mut self, canister_id: CanisterId) {
        self.canisters.remove(&canister_id);
    }

    pub fn canister_allowance(&self, canister_id: CanisterId) -> Option<Allowance> {
        self.canisters.get(&canister_id).cloned()
    }

    pub fn update_canister_allowance(
        &mut self,
        canister_id: CanisterId,
        new_allowance: SetAllowance,
    ) {
        if let Some(allowance) = self.canisters.get_mut(&canister_id) {
            allowance.update(new_allowance);
        }
    }

    pub fn insert_request(&mut self, from: CanisterId, sign_request: SignRequest) {
        self.requests.insert(from, sign_request);
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn sign_requests(&self, from: CanisterId) -> SignRequest {
        self.requests.get(&from).unwrap().clone()
    }

    pub fn connected_canisters(&self) -> CanisterHashMap {
        self.canisters.clone()
    }

    pub fn signed(&self) -> SignedTransaction {
        self.signed.clone()
    }

    pub fn keys(&self) -> PublicKeys {
        self.ledger.public_keys.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn env(&self) -> Environment {
        self.ledger.subaccount.get_env()
    }
}
