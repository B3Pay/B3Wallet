mod wasm;

use b3_user_lib::{
    account::Account,
    allowance::{CanisterId, SetAllowance},
    error::SignerError,
    ledger::config::Environment,
    ledger::keys::Addresses,
    request::SignRequest,
    signed::SignedTransaction,
    state::{State, STATE},
    types::{CanisterHashMap, CanisterStatus, UserControlArgs},
    with_account, with_account_mut, with_state, with_state_mut,
};

use ic_cdk::{
    api::{
        call::{arg_data, CallResult},
        management_canister::main::canister_status,
        management_canister::{
            main::{install_code, update_settings, UpdateSettingsArgument},
            provisional::{CanisterIdRecord, CanisterSettings},
        },
        time,
    },
    caller,
    export::{
        candid::{candid_method, export_service},
        Principal,
    },
    init, post_upgrade, pre_upgrade, query, update,
};
use std::cell::RefCell;
use wasm::Wasm;

thread_local! {
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static WASM: RefCell<Wasm> = RefCell::new(Wasm::default());
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller = caller();
    let controllers: Principal = OWNER.with(|state| state.borrow().clone());

    if caller == controllers {
        Ok(())
    } else {
        Err("Caller is not owner!".to_string())
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
pub fn get_account(account_id: String) -> CallResult<Account> {
    with_account(account_id, |account| Ok(account.clone()))?
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
pub fn get_addresses(account_id: String) -> Addresses {
    STATE.with(|s| {
        let state = s.borrow();

        state.account(&account_id).unwrap().keys().addresses
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
pub fn get_connected_canisters(account_id: String) -> CallResult<CanisterHashMap> {
    with_account(account_id, |account| Ok(account.canisters.clone()))?
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<Account> {
    STATE.with(|s| {
        let state = s.borrow();

        state.accounts()
    })
}

#[query]
#[candid_method(query)]
pub fn get_account_requests(account_id: String, canister: CanisterId) -> CallResult<SignRequest> {
    with_account(account_id, |account| {
        Ok(account.requests.get(&canister).unwrap().clone())
    })?
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn request_allowance(
    account_id: String,
    canister: CanisterId,
    allowance: SetAllowance,
) -> CallResult<()> {
    with_account_mut(account_id, |account| {
        Ok(account.insert_canister(canister, allowance))
    })?
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn change_owner(new_owner: Principal) -> CallResult<Principal> {
    OWNER.with(|s| {
        *s.borrow_mut() = new_owner;
    });

    Ok(new_owner)
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub async fn create_account(env: Option<Environment>, name: Option<String>) -> CallResult<Account> {
    let ecdsa_path = with_state(|s| s.new_ecdsa_path(env))?;

    let new_account = Account::new(ecdsa_path).await?;

    let id = with_state_mut(|s| s.insert_account(new_account, name))?;

    let account = with_account(id, |account| account.clone())?;

    Ok(account)
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn sign_request(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> CallResult<SignRequest> {
    let canister_id = caller();

    with_account_mut(account_id, |account| {
        Ok(account.new_request(canister_id, hex_raw_tx, chain_id))
    })?
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub async fn sign_message(account_id: String, message_hash: Vec<u8>) -> CallResult<Vec<u8>> {
    let account = with_account(account_id, |account| account.clone())?;

    account.sign_message(message_hash).await
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> CallResult<SignedTransaction> {
    let account = with_account(account_id, |account| account.clone())?;

    account.sign_transaction(hex_raw_tx, chain_id).await
}

#[candid_method(query)]
#[query]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn upgrade_canister() {
    let args = WASM.with(|s| s.borrow().upgrade_args().clone());

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn reset_accounts() {
    STATE.with(|s| s.borrow_mut().reset());
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn reintall_canister() {
    let args = WASM.with(|s| s.borrow().reintall_args().clone());

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn reset_wasm() {
    WASM.with(|s| s.borrow_mut().reset());
}

#[candid_method(query)]
#[query]
fn wasm_version() -> String {
    WASM.with(|s| s.borrow().version.clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn load_wasm(blob: Vec<u8>, version: String) -> CallResult<u64> {
    let mut wasm = WASM.with(|s| s.borrow().wasm.clone());

    wasm = wasm.iter().copied().chain(blob.iter().copied()).collect();

    let total = WASM.with(|s| {
        let state = &mut *s.borrow_mut();

        state.wasm = wasm;
        state.version = version;

        state.wasm.len()
    });

    Ok(total as u64)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn update_canister_controllers(mut controllers: Vec<Principal>) -> CallResult<()> {
    let canister_id = ic_cdk::id();
    let owner = OWNER.with(|s| *s.borrow());

    if !controllers.contains(&owner) {
        controllers.push(owner);
    }

    if !controllers.contains(&canister_id) {
        controllers.push(canister_id);
    }

    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    update_settings(arg).await
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn status() -> Result<CanisterStatus, SignerError> {
    let canister_id = ic_cdk::id();

    let version = version();

    let status = canister_status(CanisterIdRecord { canister_id }).await;

    match status {
        Ok((status,)) => Ok(CanisterStatus {
            id: canister_id,
            status,
            version,
            status_at: time(),
        }),
        Err((_, message)) => Err(SignerError::CanisterStatusError(message)),
    }
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let owner = OWNER.with(|s| s.borrow().clone());
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s, owner)).unwrap();
    });

    WASM.with(|s| s.borrow_mut().reset());
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

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    let mut file = std::fs::File::create("./b3_user.did").unwrap();

    let candid = export_candid();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
