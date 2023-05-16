use super::sign::{Executable, SignRequest};
use crate::{
    error::WalletError,
    signer::{Roles, Signer},
    store::{with_account_mut, with_signers_mut},
};
use b3_helper::types::{CanisterId, SignerId, Wasm, WasmHash, WasmHashString, WasmVersion};
use enum_dispatch::enum_dispatch;
use ic_cdk::{
    api::management_canister::main::UpdateSettingsArgument,
    export::{candid::CandidType, serde::Deserialize},
};

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize)]
pub enum InterCanisterRequest {
    RenameAccountRequest,
    AddSignerRequest,
    UpdateSettingsRequest,
    UpgradeCanisterRequest,
    TopUpCanisterRequest,
    RawRandRequest,
    CallRequest,
    QueryRequest,
}

// RENAME ACCOUNT - START
#[derive(CandidType, Clone, Deserialize)]
pub struct RenameAccountRequest {
    pub new_name: String,
    pub account_id: String,
}

impl From<RenameAccountRequest> for SignRequest {
    fn from(args: RenameAccountRequest) -> Self {
        SignRequest::InterCanisterRequest(InterCanisterRequest::RenameAccountRequest(args))
    }
}

impl Executable for RenameAccountRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let new_name = self.new_name.clone();

        with_account_mut(&self.account_id, |account| account.rename(new_name))?;

        Ok(())
    }
}

impl RenameAccountRequest {
    pub fn new(account_id: String, name: String) -> Self {
        Self {
            account_id,
            new_name: name,
        }
    }
}
// RENAME ACCOUNT - END

// ADD SIGNER - START
#[derive(CandidType, Clone, Deserialize)]
pub struct AddSignerRequest {
    pub name: Option<String>,
    pub role: Roles,
    pub signer_id: SignerId,
    pub expires_at: Option<u64>,
}

impl Executable for AddSignerRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let signer_id = self.signer_id.clone();
        let signer_name = self.name.clone();
        let signer_role = self.role.clone();

        let signer = Signer::new(signer_role, signer_name, self.expires_at);

        with_signers_mut(|signers| signers.insert(signer_id, signer))
            .ok_or(WalletError::UnknownError)?;

        Ok(())
    }
}

impl AddSignerRequest {
    pub fn new(
        signer_id: SignerId,
        name: Option<String>,
        role: Roles,
        expires_at: Option<u64>,
    ) -> Self {
        AddSignerRequest {
            name,
            role,
            signer_id,
            expires_at,
        }
    }
}
// ADD SIGNER - END

// UPDATE SETTINGS - START
#[derive(CandidType, Clone, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: UpdateSettingsArgument,
}

impl Executable for UpdateSettingsRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let settings = self.settings.clone();

        // ic_cdk::api::management_canister::update_settings(settings)?;

        Ok(())
    }
}

impl UpdateSettingsRequest {
    pub fn new(settings: UpdateSettingsArgument) -> Self {
        UpdateSettingsRequest { settings }
    }
}
// UPDATE SETTINGS - END

// UPGRADE CANISTER - START
#[derive(CandidType, Clone, Deserialize)]
pub struct UpgradeCanisterRequest {
    pub wasm_hash: WasmHash,
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
}

impl Executable for UpgradeCanisterRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let wasm_hash = self.wasm_hash.clone();
        let wasm_version = self.wasm_version.clone();
        let wasm_hash_string = self.wasm_hash_string.clone();

        // ic_cdk::api::canister::upgrade(wasm_hash, wasm_version, wasm_hash_string)?;

        Ok(())
    }
}

impl UpgradeCanisterRequest {
    pub fn new(wasm: Wasm, wasm_version: WasmVersion) -> Self {
        UpgradeCanisterRequest {
            wasm_version,
            wasm_hash: wasm.generate_hash(),
            wasm_hash_string: wasm.generate_hash_string(),
        }
    }
}
// UPGRADE CANISTER - END

// TOP UP CANISTER - START
#[derive(CandidType, Clone, Deserialize)]
pub struct TopUpCanisterRequest {
    pub canister_id: CanisterId,
    pub amount: u64,
}

impl Executable for TopUpCanisterRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let canister_id = self.canister_id.clone();
        let amount = self.amount;

        // ic_cdk::api::canister::add_cycles(amount)?;

        Ok(())
    }
}

impl TopUpCanisterRequest {
    pub fn new(canister_id: CanisterId, amount: u64) -> Self {
        TopUpCanisterRequest {
            canister_id,
            amount,
        }
    }
}
// TOP UP CANISTER - END

// RAW RAND - START
#[derive(CandidType, Clone, Deserialize)]
pub struct RawRandRequest {
    pub length: u32,
}

impl Executable for RawRandRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let length = self.length;

        // ic_cdk::api::raw_rand(length)?;

        Ok(())
    }
}

impl RawRandRequest {
    pub fn new(length: u32) -> Self {
        RawRandRequest { length }
    }
}
// RAW RAND - END

// CALL - START
#[derive(CandidType, Clone, Deserialize)]
pub struct CallRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
    pub cycles: Option<u64>,
}

impl Executable for CallRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let canister_id = self.canister_id.clone();
        let method_name = self.method_name.clone();
        let arg = self.arg.clone();
        let sender = self.sender.clone();
        let cycles = self.cycles;

        // ic_cdk::api::call::call(canister_id, method_name, arg, sender, cycles)?;

        Ok(())
    }
}

impl CallRequest {
    pub fn new(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
        cycles: Option<u64>,
    ) -> Self {
        CallRequest {
            canister_id,
            method_name,
            arg,
            sender,
            cycles,
        }
    }
}
// CALL - END

// QUERY - START
#[derive(CandidType, Clone, Deserialize)]
pub struct QueryRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
}

impl Executable for QueryRequest {
    fn execute(&self) -> Result<(), WalletError> {
        let canister_id = self.canister_id.clone();
        let method_name = self.method_name.clone();
        let arg = self.arg.clone();
        let sender = self.sender.clone();

        // ic_cdk::api::call::call(canister_id, method_name, arg, sender, cycles)?;

        Ok(())
    }
}

impl QueryRequest {
    pub fn new(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
    ) -> Self {
        QueryRequest {
            canister_id,
            method_name,
            arg,
            sender,
        }
    }
}
// QUERY - END
