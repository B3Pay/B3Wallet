use crate::counter::WalletCounters;
use crate::request::sign::SignRequest;
use crate::request::Request;
use crate::signed::SignedTransaction;
use crate::types::{AccountId, ConfirmedMap, RequestId, Requests};
use b3_helper::types::{AccountsCounter, Environment, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::account::WalletAccount;
use crate::error::SignerError;
use crate::ledger::public_keys::PublicKeys;
use crate::types::WalletAccountMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    counters: WalletCounters,
    confirms: ConfirmedMap,
    requests: Requests,
    accounts: WalletAccountMap,
}

impl Default for State {
    fn default() -> Self {
        State {
            counters: WalletCounters::new(),
            accounts: WalletAccountMap::new(),
            confirms: ConfirmedMap::new(),
            requests: Requests::new(),
        }
    }
}

impl State {
    // Init Functions

    pub fn init_wallet(&mut self) {
        if self.counters.total() > 0 {
            return;
        }

        let mut account = WalletAccount::from(Subaccount::default());

        account.update_name("Main Account".to_owned());

        self.accounts.insert("default".to_owned(), account);
    }

    // New Functions

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        Subaccount::new(env, counter)
    }

    pub fn new_request(&self, request: SignRequest, deadline: Option<u64>) -> Request {
        let id = self.request_counter();

        Request::new(id, request, deadline)
    }

    // Insert Functions

    pub fn insert_account(
        &mut self,
        mut account: WalletAccount,
        opt_name: Option<String>,
    ) -> AccountId {
        if let Some(name) = opt_name {
            account.update_name(name);
        } else {
            let env = account.environment();

            let name = self.counters.generate_next_account_name(env);

            account.update_name(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    pub fn insert_request(&mut self, sign_request: Request) -> RequestId {
        let id = sign_request.id();

        self.requests.insert(id.clone(), sign_request);

        id
    }

    // Account Functions

    pub fn remove_account(&mut self, id: &String) -> Result<(), SignerError> {
        if id == "default" {
            return Err(SignerError::CannotRemoveDefaultAccount);
        }

        self.accounts
            .remove(id)
            .ok_or(SignerError::WalletAccountNotExists)?;

        Ok(())
    }

    pub fn hide_account(&mut self, id: &String) -> Result<(), SignerError> {
        let account = self.account_mut(id)?;

        account.hide();

        Ok(())
    }

    pub fn unhide_account(&mut self, id: &String) -> Result<(), SignerError> {
        let account = self.account_mut(id)?;

        account.unhide();

        Ok(())
    }

    pub fn account(&self, id: &String) -> Result<&WalletAccount, SignerError> {
        self.accounts
            .get(id)
            .ok_or(SignerError::WalletAccountNotExists)
    }

    pub fn account_mut(&mut self, id: &String) -> Result<&mut WalletAccount, SignerError> {
        self.accounts
            .get_mut(id)
            .ok_or(SignerError::WalletAccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<PublicKeys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.public_keys())
            .collect()
    }

    pub fn accounts(&self) -> Vec<WalletAccount> {
        self.accounts
            .iter()
            .map(|(_, account)| account.clone())
            .collect()
    }

    pub fn accounts_len(&self) -> usize {
        self.accounts.len()
    }

    pub fn account_status(&self) -> AccountsCounter {
        self.counters.clone().into()
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        self.counters.account(env)
    }

    // Request Functions

    pub fn request_counter(&self) -> usize {
        self.counters.request
    }

    pub fn remove_request(&mut self, request_id: RequestId) {
        // remove request from requests Vec based on the id
        let index = self
            .requests
            .iter()
            .position(|request| request.id() == request_id);

        if let Some(index) = index {
            self.requests.remove(index);
        }
    }

    pub fn sign_requests(&self) -> Requests {
        self.requests.clone()
    }

    pub fn sign_request(&self, request_id: RequestId) -> Result<Request, SignerError> {
        self.requests
            .iter()
            .find(|request| request.id() == request_id)
            .cloned()
            .ok_or(SignerError::RequestNotFound(request_id))
    }

    // Confirmed Functions

    pub fn confirmed(&self, request_id: RequestId) -> Result<&SignedTransaction, SignerError> {
        self.confirms
            .get(&request_id)
            .ok_or(SignerError::RequestNotFound(request_id))
    }

    pub fn confirmed_mut(
        &mut self,
        request_id: RequestId,
    ) -> Result<&mut SignedTransaction, SignerError> {
        self.confirms
            .get_mut(&request_id)
            .ok_or(SignerError::RequestNotFound(request_id))
    }

    pub fn insert_signed_transaction(
        &mut self,
        request_id: RequestId,
        signed_tx: SignedTransaction,
    ) {
        self.confirms.insert(request_id, signed_tx);
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.requests.clear();
        self.confirms.clear();
        self.counters = WalletCounters::new();
    }
}
