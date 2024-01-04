use crate::error::WalletError;
use crate::ledger::ledger::Ledger;
use crate::nonces::NonceTrait;
use crate::setting::WalletSettings;
use crate::types::{WalletAccountMap, WalletAccountView};
use crate::{account::WalletAccount, types::AccountId};
use b3_utils::api::AppAccountsNonce;
use b3_utils::memory::types::{Bound, Storable};
use b3_utils::nonce::Nonce;
use b3_utils::Environment;
use b3_utils::Subaccount;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

mod test;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct WalletState {
    pub nonces: AppAccountsNonce,
    pub settings: WalletSettings,
    pub accounts: WalletAccountMap,
}

impl Storable for WalletState {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

impl WalletState {
    pub fn new() -> Self {
        let subaccount = Subaccount::new(Environment::Production, 0);

        let default_account = WalletAccount::new(subaccount, "Main Account".to_owned());

        let mut accounts = WalletAccountMap::default();

        accounts.insert("-default".to_owned(), default_account);

        WalletState {
            nonces: AppAccountsNonce::new(),
            settings: WalletSettings::default(),
            accounts,
        }
    }

    // Init Functions
    pub fn init_wallet(&mut self, setting: WalletSettings) {
        self.init_setting(setting);
    }

    pub fn init_setting(&mut self, setting: WalletSettings) {
        if self.settings.initialised {
            return;
        }

        self.settings = setting;
        self.settings.initialised = true;
    }

    pub fn init_accounts(&mut self) {
        if self.accounts_len() > 0 {
            return;
        }

        let subaccount = Subaccount::new(Environment::Production, 0);
        let mut account = WalletAccount::from(subaccount);

        account.rename("Main Account".to_owned());

        self.accounts.insert("-default".to_owned(), account);

        self.nonces.increment(Environment::Production);
    }

    pub fn is_initialised(&self) -> bool {
        self.settings.initialised
    }

    pub fn set_setting(&mut self, setting: WalletSettings) {
        self.settings = setting;
    }

    // Account Functions

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let nonce = self.account_nonce(&env);

        Subaccount::new(env, nonce.add_64(1).get())
    }

    pub fn insert_account(&mut self, mut account: WalletAccount, opt_name: Option<String>) {
        let env = account.environment();

        let name = self.nonces.generate_next_name(env);

        if let Some(name) = opt_name {
            account.rename(name);
        } else {
            account.rename(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);
    }

    pub fn counters(&self) -> &AppAccountsNonce {
        &self.nonces
    }

    pub fn account(&self, id: &AccountId) -> Result<&WalletAccount, WalletError> {
        self.accounts
            .get(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn account_mut(&mut self, id: &AccountId) -> Result<&mut WalletAccount, WalletError> {
        self.accounts
            .get_mut(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<&Ledger> {
        self.accounts
            .iter()
            .map(|(_, account)| account.ledger())
            .collect()
    }

    pub fn account_views(&self) -> Vec<WalletAccountView> {
        self.accounts
            .values()
            .map(WalletAccountView::from)
            .collect()
    }

    pub fn accounts_len(&self) -> usize {
        self.accounts.len()
    }

    pub fn account_status(&self) -> AppAccountsNonce {
        self.nonces.clone().into()
    }

    pub fn account_nonce(&self, env: &Environment) -> Nonce {
        self.nonces.account(env)
    }

    pub fn remove_account(&mut self, id: &String) -> Result<(), WalletError> {
        if id == "-default" {
            return Err(WalletError::CannotRemoveDefaultAccount);
        }

        self.accounts
            .remove(id)
            .ok_or(WalletError::WalletAccountNotExists)?;

        Ok(())
    }

    pub fn restore_account(&mut self, subaccount: Subaccount) -> Result<(), WalletError> {
        if self.accounts.contains_key(&subaccount.id()) {
            return Err(WalletError::WalletAccountAlreadyExists);
        }

        if self.nonces.account(&subaccount.environment()).get() <= subaccount.nonce() {
            return Err(WalletError::WalletAccountCounterMismatch);
        }

        let name = subaccount.name();

        let account = WalletAccount::from(subaccount);

        self.insert_account(account, Some(name));

        Ok(())
    }

    pub fn reset_accounts(&mut self) {
        self.accounts.clear();
        self.nonces.reset();

        self.init_accounts();
    }
}
