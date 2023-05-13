use crate::{
    allowance::SignerAllowance,
    error::SignerError,
    evm_tx::get_evm_transaction,
    ledger::{ledger::Ledger, public_keys::PublicKeys, subaccount::SubaccountTrait},
    request::EvmSignRequest,
    signed::SignedTransaction,
    types::CanisterAllowances,
    types::SignerAllowanceArgs,
};
use b3_helper::types::{CanisterId, Environment, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Deserialize)]
pub struct SignerAccount {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub ledger: Ledger,
    pub signed: SignedTransaction,
    pub requests: HashMap<CanisterId, EvmSignRequest>,
    pub canisters: HashMap<CanisterId, SignerAllowance>,
}

impl Default for SignerAccount {
    fn default() -> Self {
        SignerAccount {
            id: String::new(),
            name: String::new(),
            hidden: false,
            requests: HashMap::new(),
            canisters: HashMap::new(),
            signed: SignedTransaction::default(),
            ledger: Ledger::default(),
        }
    }
}

impl From<Subaccount> for SignerAccount {
    fn from(subaccount: Subaccount) -> Self {
        let id = subaccount.id();
        let ledger = subaccount.into();

        SignerAccount {
            id,
            ledger,
            hidden: false,
            name: String::new(),
            requests: HashMap::new(),
            canisters: HashMap::new(),
            signed: SignedTransaction::default(),
        }
    }
}

impl SignerAccount {
    pub async fn sign_eth_transaction(
        &self,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> Result<SignedTransaction, SignerError> {
        let ecdsa = self.ledger.public_keys.ecdsa()?;

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
        canister_id: CanisterId,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> EvmSignRequest {
        let request = EvmSignRequest::new(hex_raw_tx, chain_id, None);

        self.requests.insert(canister_id, request.clone());

        request
    }

    pub fn remove_request(&mut self, canister_id: CanisterId) {
        self.requests.remove(&canister_id);
    }

    pub fn sign_requests(&self, canister_id: CanisterId) -> Result<EvmSignRequest, SignerError> {
        self.requests
            .get(&canister_id)
            .cloned()
            .ok_or(SignerError::RequestNotExists)
    }

    pub fn insert_request(&mut self, canister_id: CanisterId, sign_request: EvmSignRequest) {
        self.requests.insert(canister_id, sign_request);
    }

    pub fn insert_signed_transaction(&mut self, signed_tx: SignedTransaction) {
        self.signed = signed_tx;
    }

    pub fn signed(&self) -> SignedTransaction {
        self.signed.clone()
    }

    pub fn insert_canister(&mut self, canister_id: CanisterId, new_allowance: SignerAllowanceArgs) {
        let allowance = SignerAllowance::new(new_allowance);

        self.canisters.insert(canister_id, allowance);
    }

    pub fn remove_canister(&mut self, canister_id: CanisterId) {
        self.canisters.remove(&canister_id);
    }

    pub fn canister_allowance(&self, canister_id: CanisterId) -> Option<SignerAllowance> {
        self.canisters.get(&canister_id).cloned()
    }

    pub fn update_canister_allowance(
        &mut self,
        canister_id: CanisterId,
        new_allowance: SignerAllowanceArgs,
    ) {
        if let Some(allowance) = self.canisters.get_mut(&canister_id) {
            allowance.update(new_allowance);
        }
    }

    pub fn update_name(&mut self, name: String) -> String {
        self.name = name;

        self.name.clone()
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn unhide(&mut self) {
        self.hidden = false;
    }

    pub fn connected_canisters(&self) -> CanisterAllowances {
        self.canisters.clone()
    }

    pub fn public_keys(&self) -> PublicKeys {
        self.ledger.public_keys.clone()
    }

    pub fn environment(&self) -> Environment {
        self.ledger.subaccount.environment()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
