use chain::ChainData;
#[cfg(test)]
use mocks::{ic_call, ic_timestamp};

#[cfg(not(test))]
use ic_cdk::api::call::call_with_payment as ic_call;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;
use ic_cdk::{
    export::{
        candid::CandidType,
        serde::{Deserialize, Serialize},
        Principal,
    },
    init,
};
pub mod Identifier;
pub mod allowance;
pub mod config;
#[cfg(test)]
mod mocks;
pub mod public_key;
pub mod subaccount;

mod utils;
use types::UserId;
use utils::get_address_from_public_key;

pub mod account;
pub mod chain;
pub mod ecdsa;
pub mod types;

pub mod state;
use state::{State, STATE};

pub mod transaction;
use transaction::*;

use crate::types::SignedTransaction;

#[derive(CandidType, Serialize, Debug)]
pub struct CreateAddressResponse {
    pub address: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct DeployContractResponse {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct TransferERC20Response {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct UserResponse {
    pub address: String,
    pub transactions: ChainData,
}

#[init]
pub fn init(user_id: UserId) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.init(user_id);
    });
}

pub async fn sign_transaction(
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
    account_id: u8,
) -> Result<SignedTransaction, String> {
    let account = STATE.with(|s| {
        let state = s.borrow();

        state.account(account_id)
    });

    if let Some(mut account) = account {
        Ok(account.sign_transaction(hex_raw_tx, chain_id).await)
    } else {
        Err("account does not exist".to_string())
    }
}

pub async fn deploy_contract(
    principal_id: Principal,
    bytecode: Vec<u8>,
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
) -> Result<DeployContractResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }
    let data = "0x".to_owned() + &utils::vec_u8_to_string(&bytecode);
    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: "0x".to_string(),
        value: 0,
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();
    let res = sign_transaction(raw_tx, chain_id, principal_id)
        .await
        .unwrap();

    Ok(DeployContractResponse { tx: res.sign_tx })
}

pub async fn transfer_erc_20(
    principal_id: Principal,
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
    address: String,
    value: u64,
    contract_address: String,
) -> Result<TransferERC20Response, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }

    let data = "0x".to_owned() + &utils::get_transfer_data(&address, value).unwrap();

    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: contract_address,
        value: 0,
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();

    let res = sign_transaction(raw_tx, chain_id, principal_id)
        .await
        .unwrap();

    Ok(TransferERC20Response { tx: res.sign_tx })
}

pub fn get_caller_data(principal_id: Principal, chain_id: u64) -> Option<UserResponse> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;
    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return None;
    }

    let address = get_address_from_public_key(user.public_key.clone()).unwrap();

    let transaction_data = user
        .transactions
        .get(&chain_id)
        .cloned()
        .unwrap_or_else(|| ChainData::default());

    Some(UserResponse {
        address,
        transactions: transaction_data,
    })
}

pub fn clear_caller_history(principal_id: Principal, chain_id: u64) -> Result<(), String> {
    let users = STATE.with(|s| s.borrow().users.clone());

    if let None = users.get(&principal_id) {
        return Err("this user does not exist".to_string());
    }

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();
        let user_tx = user.transactions.get_mut(&chain_id);
        if let Some(user_transactions) = user_tx {
            user_transactions.transactions.clear();
        }
    });

    Ok(())
}

pub fn pre_upgrade() {
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s,)).unwrap();
    });
}

pub fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });
}
#[cfg(test)]
mod tests;
