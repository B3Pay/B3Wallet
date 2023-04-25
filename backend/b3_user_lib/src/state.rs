use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use ic_cdk::trap;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::account::Account;
use crate::allowance::{Allowance, CanisterId, SetAllowance};
use crate::keys::Keys;
use crate::subaccount::Subaccount;
use crate::types::SignRequest;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct State {
    subaccount: Subaccount,
    accounts: Vec<Account>,
    requested_signatures: Vec<SignRequest>,
    connected_canisters: HashMap<CanisterId, Allowance>,
}

impl Default for State {
    fn default() -> Self {
        State {
            subaccount: Subaccount::default(),
            accounts: Vec::with_capacity(256),
            requested_signatures: Vec::new(),
            connected_canisters: HashMap::new(),
        }
    }
}

impl State {
    pub fn init(&mut self, caller: Principal) {
        if self.accounts.len() > 0 {
            trap("State already initialized!");
        }

        self.subaccount = Subaccount::new(&caller);
    }

    pub fn insert_account(&mut self, mut account: Account, name: Option<String>) -> u8 {
        let id = self.next_account_id();

        if let Some(name) = name {
            account.update_name(name);
        } else {
            account.update_name(format!("Account {}", id));
        }

        if self.accounts.len() == self.accounts.capacity() {
            trap("Maximum number of accounts reached!");
        }

        self.accounts.push(account);

        id
    }

    pub fn connect_canister(&mut self, canister_id: CanisterId, new_allowance: &SetAllowance) {
        let allowance = Allowance::new(new_allowance);

        self.connected_canisters.insert(canister_id, allowance);
    }

    pub fn update_canister_allowance(
        &mut self,
        canister_id: CanisterId,
        new_allowance: &SetAllowance,
    ) {
        if let Some(allowance) = self.connected_canisters.get_mut(&canister_id) {
            allowance.update(new_allowance);
        } else {
            trap(&format!("Canister {} is not connected!", canister_id));
        }
    }

    pub fn request_signature(&mut self, canister_id: CanisterId, sign_request: SignRequest) {
        if let Some(allowance) = self.connected_canisters.get_mut(&canister_id) {
            if allowance.is_allowed() {
                if let Some(limit) = allowance.limit {
                    if limit == 0 {
                        trap(&format!("Canister {} has no more allowance!", canister_id));
                    }
                    allowance.decrease_limit();
                }
                self.requested_signatures.push(sign_request);
            } else {
                trap(&format!(
                    "Canister {} is not authorized to request a signature!",
                    canister_id
                ));
            }
        } else {
            trap(&format!("Canister {} is not connected!", canister_id));
        }
    }

    pub fn drivation_path(&self, id: u8) -> Vec<u8> {
        self.subaccount.derive_hd_path(id)
    }

    pub fn new_drivation_path(&self) -> Vec<u8> {
        if self.accounts.len() == self.accounts.capacity() {
            trap("Maximum number of accounts reached!");
        }

        let id = self.next_account_id();

        self.subaccount.derive_hd_path(id)
    }

    fn next_account_id(&self) -> u8 {
        self.accounts.len() as u8
    }

    pub fn account(&self, id: u8) -> Option<Account> {
        self.accounts.get(id as usize).cloned()
    }

    pub fn account_key(&self, id: u8) -> Option<Keys> {
        self.account(id).map(|account| account.keys())
    }

    pub fn account_keys(&self) -> Vec<Keys> {
        self.accounts.iter().map(|account| account.keys()).collect()
    }

    pub fn connected_canister(&self, canister_id: CanisterId) -> Option<Allowance> {
        self.connected_canisters.get(&canister_id).cloned()
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    pub fn requested_signatures(&self) -> Vec<SignRequest> {
        self.requested_signatures.clone()
    }

    pub fn connected_canisters(&self) -> HashMap<CanisterId, Allowance> {
        self.connected_canisters.clone()
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
