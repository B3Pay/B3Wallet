use candid::{candid_method, Principal};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};

use b3_user_lib::{
    account::Account,
    config::Environment,
    public_key::PublicKey,
    signed::SignedTransaction,
    state::{State, STATE},
};

#[init]
#[candid_method(init)]
pub fn init() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.init(Principal::anonymous());
    });
}

#[query]
#[candid_method(query)]
pub fn get_account(account_id: u8) -> Account {
    STATE.with(|s| {
        let state = s.borrow();

        state.account(account_id).unwrap()
    })
}

#[query]
#[candid_method(query)]
pub fn get_public_key(account_id: u8) -> PublicKey {
    STATE.with(|s| {
        let state = s.borrow();

        state.public_key(account_id).unwrap()
    })
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<Account> {
    STATE.with(|s| {
        let state = s.borrow();

        state.accounts()
    })
}

#[update]
#[candid_method(update)]
pub async fn create_account(env: Environment, name: Option<String>) -> Result<PublicKey, String> {
    let drivation_path = STATE.with(|s| s.borrow().new_drivation_path());

    let account = Account::new(drivation_path, env).await;

    let public_data = STATE.with(|s| {
        let mut state = s.borrow_mut();

        let id = state.insert_account(account, name);

        state.public_key(id)
    });

    if let Some(public_data) = public_data {
        Ok(public_data)
    } else {
        Err("Failed to create account".to_string())
    }
}

#[update]
#[candid_method(update)]
pub async fn sign_transaction(
    account_id: u8,
    chain_id: u64,
    hex_raw_tx: Vec<u8>,
) -> Result<SignedTransaction, String> {
    let account = STATE.with(|s| {
        let state = s.borrow();

        state.account(account_id)
    });

    if let Some(account) = account {
        let tx = account.new_transaction(hex_raw_tx, chain_id).await;

        STATE.with(|s| {
            let state = s.borrow_mut();

            let mut account = state.account(account_id).unwrap();

            account.insert_transaction(chain_id, tx.clone());
        });

        Ok(tx)
    } else {
        Err(format!("account does not exist: {}", account_id))
    }
}

#[pre_upgrade]
pub fn pre_upgrade() {
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s,)).unwrap();
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    candid::export_service!();

    let candid = format!("{}", __export_service());

    let mut file = std::fs::File::create("./b3_user.did").unwrap();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
