use b3_user_lib::state::STATE;
use b3_user_lib::types::{CanisterStatus, UserControlArgs};
use ic_cdk::api::call::arg_data;
use ic_cdk::api::management_canister::main::canister_status;
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;
use ic_cdk::api::time;
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, init, post_upgrade, pre_upgrade, query, update};

use b3_user_lib::{
    account::Account, config::Environment, keys::Keys, signed::SignedTransaction, state::State,
};

use std::cell::RefCell;

thread_local! {
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller = caller();
    let controllers: Principal = OWNER.with(|state| state.borrow().clone());

    if caller == controllers {
        Ok(())
    } else {
        Err("Caller is not the owner.".to_string())
    }
}

#[init]
#[candid_method(init)]
pub fn init() {
    let call_arg = arg_data::<(Option<UserControlArgs>,)>().0;

    let owner = match call_arg {
        Some(args) => args.owner,
        None => caller(),
    };

    OWNER.with(|s| {
        *s.borrow_mut() = owner;
    });
}

#[query]
#[candid_method(query)]
pub fn get_caller() -> Principal {
    caller()
}

#[query]
#[candid_method(query)]
pub fn get_owner() -> Principal {
    OWNER.with(|s| s.borrow().clone())
}

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> Account {
    STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id).unwrap()
    })
}

#[query]
#[candid_method(query)]
pub fn number_of_accounts() -> u8 {
    STATE.with(|s| {
        let state = s.borrow();

        state.accounts_len() as u8
    })
}

#[query]
#[candid_method(query)]
pub fn get_public_key(account_id: String) -> Keys {
    STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id).unwrap().keys()
    })
}

#[query]
#[candid_method(query)]
pub fn get_signed(account_id: String) -> SignedTransaction {
    STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id).unwrap().signed()
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

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn change_owner(new_owner: Principal) {
    OWNER.with(|s| {
        *s.borrow_mut() = new_owner;
    });
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub async fn create_account(
    env: Option<Environment>,
    name: Option<String>,
) -> Result<Account, String> {
    let ecdsa_path = STATE.with(|s| s.borrow().new_ecdsa_path(env));

    let account = Account::new(ecdsa_path).await;

    let state_account = STATE.with(|s| {
        let mut state = s.borrow_mut();

        let id = state.insert_account(account, name);

        state.account(&id)
    });

    if let Some(state_account) = state_account {
        Ok(state_account.clone())
    } else {
        Err("Failed to create account".to_string())
    }
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub async fn sign_message(account_id: String, message_hash: Vec<u8>) -> Result<Vec<u8>, String> {
    let account = STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id)
    });

    if let Some(account) = account {
        let signature = account.sign_message(message_hash).await;

        Ok(signature)
    } else {
        Err(format!("account does not exist: {}", account_id))
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn get_balance(account_id: String) -> Result<u64, String> {
    let account = STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id)
    });

    if let Some(account) = account {
        Ok(account.keys().btc_balance().await)
    } else {
        Err(format!("account does not exist: {}", account_id))
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> Result<SignedTransaction, String> {
    let account = STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id)
    });

    if let Some(account) = account {
        let tx = account.sign_transaction(hex_raw_tx, chain_id).await;

        Ok(tx)
    } else {
        Err(format!("account does not exist: {}", account_id))
    }
}

#[candid_method(query)]
#[query]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn status() -> Result<CanisterStatus, String> {
    let canister_id = ic_cdk::id();

    let status = canister_status(CanisterIdRecord { canister_id }).await;

    match status {
        Ok((status,)) => Ok(CanisterStatus {
            id: canister_id,
            status,
            status_at: time(),
        }),
        Err((_, message)) => Err(["Failed to get canister status: ".to_string(), message].join("")),
    }
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let owner = OWNER.with(|s| s.borrow().clone());
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s, owner)).unwrap();
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev, owner_prev): (State, Principal) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });

    OWNER.with(|s| {
        *s.borrow_mut() = owner_prev;
    });
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    ic_cdk::export::candid::export_service!();

    let candid = format!("{}", __export_service());

    let mut file = std::fs::File::create("./b3_user.did").unwrap();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
