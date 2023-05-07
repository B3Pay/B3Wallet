use crate::{
    allowance::Allowance,
    error::SignerError,
    evm_tx::get_evm_transaction,
    ledger::{
        config::Environment, ledger::Ledger, public_keys::PublicKeys, subaccount::Subaccount,
    },
    request::SignRequest,
    signed::SignedTransaction,
    types::CanisterAllowances,
    types::{CanisterId, SetAllowance},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Deserialize)]
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

    pub async fn sign_transaction(
        &self,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> Result<SignedTransaction, SignerError> {
        let ecdsa = self.ledger.public_keys.get_ecdsa()?;

        let mut evm_tx =
            get_evm_transaction(&hex_raw_tx, chain_id).map_err(|e| SignerError::InvalidTx(e))?;

        let message = evm_tx
            .get_message_to_sign()
            .map_err(|e| SignerError::InvalidMsg(e))?;

        if message.len() != 32 {
            return Err(SignerError::InvalidMessageLength);
        }

        let signature = self.ledger.sign_with_ecdsa(message).await?;

        let signed_evm_tx = evm_tx
            .sign(signature, ecdsa)
            .map_err(|e| SignerError::InvalidSignature(e))?;

        let signed_tx = SignedTransaction::new(signed_evm_tx);

        Ok(signed_tx)
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

    pub fn update_name(&mut self, name: String) -> String {
        self.name = name;

        self.name.clone()
    }

    pub fn sign_requests(&self, from: CanisterId) -> SignRequest {
        self.requests.get(&from).unwrap().clone()
    }

    pub fn connected_canisters(&self) -> CanisterAllowances {
        self.canisters.clone()
    }

    pub fn signed(&self) -> SignedTransaction {
        self.signed.clone()
    }

    pub fn public_keys(&self) -> PublicKeys {
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
