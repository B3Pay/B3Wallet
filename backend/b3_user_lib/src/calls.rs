use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use ic_evm_sign::state::{ChainData, Environment, State, STATE};
use std::cell::RefCell;
use std::collections::HashMap;

pub mod address;
mod types;
use crate::types::*;

#[derive(Debug, CandidType)]
struct CreateAddressResponse {
    address: String,
}
#[derive(Debug, CandidType)]
struct SignTransactionResponse {
    sign_tx: Vec<u8>,
    sign_cycles: u128,
}

#[derive(Debug, CandidType)]
struct DeployEVMContractResponse {
    tx: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct UserResponse {
    address: String,
    transactions: ChainData,
    cycles_balance: u128,
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct CanisterState {
    pub user_balances: HashMap<Principal, u128>,
}
#[derive(CandidType, Deserialize)]
struct StableState {
    canister_state: CanisterState,
    state: State,
}

thread_local! {
    pub static CANISTER_STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

#[init]
fn init(evn_opt: Option<Environment>) {
    ic_evm_sign::init(evn_opt);
}

#[update]
async fn create_address(index: u8) -> Result<CreateAddressResponse, String> {
    let principal = get_principal(index).unwrap();

    let res = ic_evm_sign::create_address(principal)
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e))
        .unwrap();

    Ok(CreateAddressResponse {
        address: res.address,
    })
}

#[update]
async fn sign_evm_tx(
    index: u8,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> Result<SignTransactionResponse, String> {
    let principal = get_principal(index).unwrap();
    let canister_state = CANISTER_STATE.with(|s| s.borrow().clone());
    let user_balance;

    if let Some(user) = canister_state.user_balances.get(&principal) {
        user_balance = user.to_owned();
    } else {
        user_balance = 0;
    }

    let config = STATE.with(|s| s.borrow().config.clone());
    let sign_cycles = u128::try_from(config.sign_cycles).unwrap();
    if user_balance < sign_cycles {
        return Err("Not enough cycles".to_string());
    }

    let res = ic_evm_sign::sign_transaction(hex_raw_tx, chain_id, principal)
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
        .unwrap();

    CANISTER_STATE.with(|s| {
        let mut state = s.borrow_mut();

        if let Some(user_balance) = state.user_balances.get_mut(&principal) {
            *user_balance = *user_balance - sign_cycles;
        }
    });

    Ok(SignTransactionResponse {
        sign_tx: res.sign_tx,
        sign_cycles,
    })
}

#[update]
fn clear_caller_history(index: u8, chain_id: u64) -> Result<(), String> {
    let principal = get_principal(index).unwrap();

    let res = ic_evm_sign::clear_caller_history(principal, chain_id)
        .map_err(|e| format!("Failed to call clear_caller_history {}", e))
        .unwrap();

    Ok(res)
}

#[update]
async fn convert_to_cycles(index: u8) -> Result<u128, String> {
    let principal = get_principal(index).unwrap();

    let config = STATE.with(|s| s.borrow().config.clone());
    let cycles;

    if config.env == Environment::Development {
        cycles = u128::try_from(config.sign_cycles).unwrap();
    } else {
        cycles = transfer_and_notify().await.unwrap();
    }

    Ok(update_user_cycles(principal, cycles))
}

#[query]
fn get_caller_data(index: u8, chain_id: u64) -> Option<UserResponse> {
    let principal = get_principal(index).unwrap();

    let state = CANISTER_STATE.with(|s| s.borrow().clone());

    let cycles_balance = state.user_balances.get(&principal).unwrap_or(&0);

    let res = ic_evm_sign::get_caller_data(principal, chain_id);

    if let Some(caller) = res {
        Some(UserResponse {
            address: caller.address,
            transactions: caller.transactions,
            cycles_balance: cycles_balance.to_owned(),
        })
    } else {
        None
    }
}
#[query]
fn get_vector_of_principal(index: u8) -> Vec<u8> {
    let principal = get_principal(index).unwrap();

    principal.as_slice().to_vec()
}

fn get_principal(index: u8) -> Result<Principal, String> {
    let principal = ic_cdk::caller();

    let mut principal_with_index = principal.as_slice().to_vec();
    let latest_index = principal_with_index.len() - 1;

    principal_with_index[latest_index] = get_current_index(principal) + index;

    Principal::try_from(principal_with_index)
        .map_err(|e| format!("Failed to convert principal {}", e))
}

#[query]
fn get_address_data(chain_id: u64) -> Vec<UserResponse> {
    let mut addresses_data = Vec::new();

    for i in 0..254 {
        let res = get_caller_data(i, chain_id);

        match res {
            Some(caller) => addresses_data.push(caller),
            None => {
                ic_cdk::println!("No more address in the index {}", i);
                break;
            }
        }
    }

    addresses_data
}

fn get_current_index(principal: Principal) -> u8 {
    let principal_with_index = principal.as_slice().to_vec();
    let principal_with_index_len = principal_with_index.len();

    principal_with_index[principal_with_index_len - 1]
}

async fn get_balance(caller: Principal) -> Tokens {
    let canister_id = ic_cdk::id();

    let subaccount = subaccount_from_principal(&caller);

    let account = AccountIdentifier::new(&canister_id, &subaccount);

    let account_balance_args = AccountBalanceArgs { account: account };

    let account_balance_result: (Tokens,) = ic_cdk::call(
        MAINNET_LEDGER_CANISTER_ID,
        "account_balance",
        (account_balance_args,),
    )
    .await
    .map_err(|(code, msg)| format!("Account balance error: {}: {}", code as u8, msg))
    .unwrap();

    account_balance_result.0
}

async fn transfer_and_notify() -> Result<u128, String> {
    let cmc_canister_id = MAINNET_CYCLES_MINTING_CANISTER_ID;
    let canister_id = ic_cdk::id();
    let caller = ic_cdk::caller();

    let subaccount_caller = subaccount_from_principal(&caller);
    let subaccount_canister = subaccount_from_principal(&canister_id);

    let balance = get_balance(caller).await;
    let transfer_fee = Tokens { e8s: 10_000 };

    let amount = balance.e8s - transfer_fee.e8s;

    let transfer_args = TransferArgs {
        memo: Memo(1347768404),
        amount: Tokens { e8s: amount },
        fee: transfer_fee,
        from_subaccount: Some(subaccount_caller),
        to: AccountIdentifier::new(&cmc_canister_id, &subaccount_canister),
        created_at_time: None,
    };

    let transfer_result: (TransferResult,) =
        ic_cdk::call(MAINNET_LEDGER_CANISTER_ID, "transfer", (transfer_args,))
            .await
            .map_err(|(code, msg)| format!("Transfer error: {}: {}", code as u8, msg))
            .unwrap();

    let transfer_block = transfer_result.0.unwrap();

    // notify top_up
    let notify_args = NotifyTopupArgs {
        block_index: transfer_block,
        canister_id: ic_cdk::id(),
    };
    let (notify_enum,): (NotifyTopUpResult,) =
        ic_cdk::call(cmc_canister_id, "notify_top_up", (notify_args,))
            .await
            .map_err(|(code, msg)| format!("Notify topup  error: {}: {}", code as u8, msg))
            .unwrap();

    let notify_result = match &notify_enum {
        NotifyTopUpResult::Ok(x) => Ok(x),
        NotifyTopUpResult::Err(x) => Err(x),
    };
    Ok(notify_result.unwrap().clone())
}

fn subaccount_from_principal(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}

fn update_user_cycles(user: Principal, cycles: u128) -> u128 {
    CANISTER_STATE.with(|s| {
        let mut state = s.borrow_mut();

        if let Some(user_cycles) = state.user_balances.get_mut(&user) {
            *user_cycles = *user_cycles + cycles;

            user_cycles.to_owned()
        } else {
            state.user_balances.insert(user, cycles);

            cycles
        }
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|s| s.borrow().clone());
    let canister_state = CANISTER_STATE.with(|s| s.borrow().clone());
    let stable_state = StableState {
        state,
        canister_state,
    };

    ic_cdk::storage::stable_save((stable_state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (s_prev,): (StableState,) = ic_cdk::storage::stable_restore().unwrap();

    CANISTER_STATE.with(|s| {
        *s.borrow_mut() = s_prev.canister_state;
    });

    STATE.with(|s| {
        *s.borrow_mut() = s_prev.state;
    });
}
