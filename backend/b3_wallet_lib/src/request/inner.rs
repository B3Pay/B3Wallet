use b3_helper::types::{CanisterId, Wasm, WasmHash, WasmHashString, WasmVersion};
use ic_cdk::{
    api::management_canister::main::UpdateSettingsArgument,
    export::{candid::CandidType, serde::Deserialize},
};

#[derive(CandidType, Clone, Deserialize)]
pub enum InnerCanisterRequest {
    RenameAccount(RenameAccountRequest),
    AddSigner(AddSignerRequest),
    UpdateSettings(UpdateSettingsRequest),
    UpdateCanister(UpgradeCanisterRequest),
    TopUpCanister(TopUpCanisterRequest),
    RawRand(RawRandRequest),
    Call(CallRequest),
    Query(QueryRequest),
}

impl InnerCanisterRequest {
    pub fn new_rename_account(account_id: String, name: String) -> Self {
        InnerCanisterRequest::RenameAccount(RenameAccountRequest::new(account_id, name))
    }

    pub fn new_add_signer(
        name: String,
        role: String,
        canister_id: CanisterId,
        expires_at: Option<u64>,
    ) -> Self {
        InnerCanisterRequest::AddSigner(AddSignerRequest::new(name, role, canister_id, expires_at))
    }

    pub fn new_update_settings(settings: UpdateSettingsArgument) -> Self {
        InnerCanisterRequest::UpdateSettings(UpdateSettingsRequest::new(settings))
    }

    pub fn new_upgrade_canister(wasm: Wasm, wasm_version: WasmVersion) -> Self {
        InnerCanisterRequest::UpdateCanister(UpgradeCanisterRequest::new(wasm, wasm_version))
    }

    pub fn new_top_up_canister(canister_id: CanisterId, amount: u64) -> Self {
        InnerCanisterRequest::TopUpCanister(TopUpCanisterRequest::new(canister_id, amount))
    }

    pub fn new_raw_rand(length: u32) -> Self {
        InnerCanisterRequest::RawRand(RawRandRequest::new(length))
    }

    pub fn new_call(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
        cycles: Option<u64>,
        deadline: u64,
    ) -> Self {
        InnerCanisterRequest::Call(CallRequest::new(
            canister_id,
            method_name,
            arg,
            sender,
            cycles,
            deadline,
        ))
    }

    pub fn new_query(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
        deadline: u64,
    ) -> Self {
        InnerCanisterRequest::Query(QueryRequest::new(
            canister_id,
            method_name,
            arg,
            sender,
            deadline,
        ))
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct RenameAccountRequest {
    name: String,
    deadline: u64,
    account_id: String,
}

impl RenameAccountRequest {
    pub fn new(account_id: String, name: String) -> Self {
        Self {
            account_id,
            name,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct AddSignerRequest {
    pub name: String,
    pub role: String,
    pub canister_id: CanisterId,
    pub expires_at: Option<u64>,
    pub deadline: u64,
}

impl AddSignerRequest {
    pub fn new(
        name: String,
        role: String,
        canister_id: CanisterId,
        expires_at: Option<u64>,
    ) -> Self {
        AddSignerRequest {
            name,
            role,
            canister_id,
            expires_at,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: UpdateSettingsArgument,
    pub deadline: u64,
}

impl UpdateSettingsRequest {
    pub fn new(settings: UpdateSettingsArgument) -> Self {
        UpdateSettingsRequest {
            settings,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct UpgradeCanisterRequest {
    pub wasm_hash: WasmHash,
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
    pub deadline: u64,
}

impl UpgradeCanisterRequest {
    pub fn new(wasm: Wasm, wasm_version: WasmVersion) -> Self {
        UpgradeCanisterRequest {
            wasm_version,
            wasm_hash: wasm.generate_hash(),
            wasm_hash_string: wasm.generate_hash_string(),
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct TopUpCanisterRequest {
    pub canister_id: CanisterId,
    pub amount: u64,
    pub deadline: u64,
}

impl TopUpCanisterRequest {
    pub fn new(canister_id: CanisterId, amount: u64) -> Self {
        TopUpCanisterRequest {
            canister_id,
            amount,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct RawRandRequest {
    pub length: u32,
    pub deadline: u64,
}

impl RawRandRequest {
    pub fn new(length: u32) -> Self {
        RawRandRequest {
            length,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CallRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
    pub cycles: Option<u64>,
    pub deadline: u64,
}

impl CallRequest {
    pub fn new(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
        cycles: Option<u64>,
        deadline: u64,
    ) -> Self {
        CallRequest {
            canister_id,
            method_name,
            arg,
            sender,
            cycles,
            deadline,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct QueryRequest {
    pub canister_id: CanisterId,
    pub method_name: String,
    pub arg: Vec<u8>,
    pub sender: Option<CanisterId>,
    pub deadline: u64,
}

impl QueryRequest {
    pub fn new(
        canister_id: CanisterId,
        method_name: String,
        arg: Vec<u8>,
        sender: Option<CanisterId>,
        deadline: u64,
    ) -> Self {
        QueryRequest {
            canister_id,
            method_name,
            arg,
            sender,
            deadline,
        }
    }
}