use b3_helper::types::{CanisterId, SignerId, Wasm, WasmHash, WasmHashString, WasmVersion};
use ic_cdk::{
    api::management_canister::main::UpdateSettingsArgument,
    export::{candid::CandidType, serde::Deserialize},
};

use crate::{
    error::WalletError,
    signer::{Roles, Signer},
    store::{with_account_mut, with_signers_mut},
};

use super::{sign::SignRequest, Executable};

#[derive(CandidType, Clone, Deserialize)]
pub enum InterCanisterRequest {
    RenameAccount(RenameAccountRequest),
    AddSigner(AddSignerRequest),
    UpdateSettings(UpdateSettingsRequest),
    UpdateCanister(UpgradeCanisterRequest),
    TopUpCanister(TopUpCanisterRequest),
    RawRand(RawRandRequest),
    Call(CallRequest),
    Query(QueryRequest),
}

impl Executable for InterCanisterRequest {
    fn execute(&self) -> Result<(), WalletError> {
        match self {
            InterCanisterRequest::RenameAccount(args) => args.execute(),
            _ => todo!("not implemented"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct RenameAccountRequest {
    pub new_name: String,
    pub account_id: String,
}

impl From<RenameAccountRequest> for SignRequest {
    fn from(args: RenameAccountRequest) -> Self {
        SignRequest::InnerCanister(InterCanisterRequest::RenameAccount(args))
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

#[derive(CandidType, Clone, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: UpdateSettingsArgument,
}

impl UpdateSettingsRequest {
    pub fn new(settings: UpdateSettingsArgument) -> Self {
        UpdateSettingsRequest { settings }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct UpgradeCanisterRequest {
    pub wasm_hash: WasmHash,
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
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

#[derive(CandidType, Clone, Deserialize)]
pub struct TopUpCanisterRequest {
    pub canister_id: CanisterId,
    pub amount: u64,
}

impl TopUpCanisterRequest {
    pub fn new(canister_id: CanisterId, amount: u64) -> Self {
        TopUpCanisterRequest {
            canister_id,
            amount,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct RawRandRequest {
    pub length: u32,
}

impl RawRandRequest {
    pub fn new(length: u32) -> Self {
        RawRandRequest { length }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CallRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
    pub cycles: Option<u64>,
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

#[derive(CandidType, Clone, Deserialize)]
pub struct QueryRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
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
