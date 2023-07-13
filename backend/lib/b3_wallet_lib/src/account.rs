use crate::{ledger::ledger::Ledger, types::WalletAccountView};
use b3_helper_lib::{environment::Environment, subaccount::Subaccount, types::Metadata};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

impl From<&WalletAccount> for WalletAccountView {
    fn from(account: &WalletAccount) -> Self {
        Self {
            id: account.id.clone(),
            name: account.name.clone(),
            hidden: account.hidden,
            metadata: account.metadata.clone(),
            environment: account.environment().clone(),
            pendings: account.ledger.pendings(),
            addresses: account.ledger.address_map().clone(),
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccount {
    id: String,
    name: String,
    hidden: bool,
    ledger: Ledger,
    metadata: Metadata,
}

impl Default for WalletAccount {
    fn default() -> Self {
        WalletAccount {
            id: String::new(),
            name: String::new(),
            hidden: false,
            metadata: Metadata::default(),
            ledger: Ledger::default(),
        }
    }
}

impl From<Subaccount> for WalletAccount {
    fn from(subaccount: Subaccount) -> Self {
        let id = subaccount.id();
        let ledger = Ledger::from(subaccount);

        WalletAccount {
            id,
            ledger,
            hidden: false,
            name: String::new(),
            metadata: Metadata::default(),
        }
    }
}

impl WalletAccount {
    pub fn new(subaccount: Subaccount, name: String) -> Self {
        let id = subaccount.id();
        let ledger = Ledger::from(subaccount.clone());

        WalletAccount {
            id,
            name,
            ledger,
            hidden: false,
            metadata: Metadata::default(),
        }
    }

    pub fn view(&self) -> WalletAccountView {
        WalletAccountView {
            id: self.id.clone(),
            name: self.name.clone(),
            hidden: self.hidden,
            metadata: self.metadata.clone(),
            addresses: self.ledger.address_map(),
            pendings: self.ledger.pendings(),
            environment: self.ledger.subaccount.environment(),
        }
    }

    pub fn subaccount(&self) -> Subaccount {
        self.ledger.subaccount.clone()
    }

    pub fn environment(&self) -> Environment {
        self.ledger.subaccount.environment()
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn ledger_mut(&mut self) -> &mut Ledger {
        &mut self.ledger
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn update_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &String) {
        self.metadata.remove(key);
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn unhide(&mut self) {
        self.hidden = false;
    }
}
