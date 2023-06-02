use crate::{
    error::WalletError,
    ledger::{evm::sign::get_evm_transaction, keys::Keys, subaccount::SubaccountTrait, Ledger},
    types::WalletAccountView,
};
use b3_helper_lib::types::{Environment, Metadata, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

impl From<&WalletAccount> for WalletAccountView {
    fn from(account: &WalletAccount) -> Self {
        Self {
            id: account.id.clone(),
            name: account.name.clone(),
            hidden: account.hidden,
            metadata: account.metadata.clone(),
            environment: account.environment().clone(),
            addresses: account.ledger.keys.addresses().clone(),
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
    pub async fn sign_evm_transaction(
        &self,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> Result<Vec<u8>, WalletError> {
        let ecdsa = self.ledger.keys.ecdsa()?;

        let mut evm_tx = get_evm_transaction(&hex_raw_tx, chain_id)?;

        let message = evm_tx.get_message_to_sign()?;

        if message.len() != 32 {
            return Err(WalletError::InvalidMessageLength);
        }

        let signature = self.ledger.sign_with_ecdsa(message).await?;

        let signed_evm_tx = evm_tx.sign(signature, ecdsa.to_vec())?;

        Ok(signed_evm_tx)
    }

    pub fn view(&self) -> WalletAccountView {
        WalletAccountView {
            id: self.id.clone(),
            name: self.name.clone(),
            hidden: self.hidden,
            metadata: self.metadata.clone(),
            addresses: self.ledger.keys.addresses().clone(),
            environment: self.ledger.subaccount.environment(),
        }
    }

    pub fn public_keys(&self) -> &Keys {
        &self.ledger.keys
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
