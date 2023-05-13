use b3_helper::types::{CanisterId, Wasm, WasmHash, WasmHashString, WasmVersion};
use ic_cdk::{
    api::management_canister::main::UpdateSettingsArgument,
    export::{candid::CandidType, serde::Deserialize},
};

use super::sign::SignRequestTrait;

#[derive(CandidType, Clone, Deserialize)]
pub enum InnerCanisterRequest {
    AddSigner(AddSignerRequest),
    UpdateSettings(UpdateSettingsRequest),
    UpdateCanister(UpgradeCanisterRequest),
    TopUpCanister(TopUpCanisterRequest),
    RawRand(RawRandRequest),
    Call(CallRequest),
    Query(QueryRequest),
}

impl SignRequestTrait for InnerCanisterRequest {
    fn get_id(&self) -> String {
        match self {
            InnerCanisterRequest::AddSigner(add_signer_request) => add_signer_request.id.clone(),
            InnerCanisterRequest::UpdateSettings(update_settings_request) => {
                update_settings_request.id.clone()
            }
            InnerCanisterRequest::UpdateCanister(upgrade_canister_request) => {
                upgrade_canister_request.id.clone()
            }
            InnerCanisterRequest::TopUpCanister(top_up_canister_request) => {
                top_up_canister_request.id.clone()
            }
            InnerCanisterRequest::RawRand(raw_rand_request) => raw_rand_request.id.clone(),
            InnerCanisterRequest::Call(call_request) => call_request.id.clone(),
            InnerCanisterRequest::Query(query_request) => query_request.id.clone(),
        }
    }

    fn get_deadline(&self) -> u64 {
        match self {
            InnerCanisterRequest::AddSigner(add_signer_request) => add_signer_request.deadline,
            InnerCanisterRequest::UpdateSettings(update_settings_request) => {
                update_settings_request.deadline
            }
            InnerCanisterRequest::UpdateCanister(upgrade_canister_request) => {
                upgrade_canister_request.deadline
            }
            InnerCanisterRequest::TopUpCanister(top_up_canister_request) => {
                top_up_canister_request.deadline
            }
            InnerCanisterRequest::RawRand(raw_rand_request) => raw_rand_request.deadline,
            InnerCanisterRequest::Call(call_request) => call_request.deadline,
            InnerCanisterRequest::Query(query_request) => query_request.deadline,
        }
    }
}

impl InnerCanisterRequest {
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
pub struct AddSignerRequest {
    pub id: String,
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
            id: "".to_string(),
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
    pub id: String,
    pub settings: UpdateSettingsArgument,
    pub deadline: u64,
}

impl UpdateSettingsRequest {
    pub fn new(settings: UpdateSettingsArgument) -> Self {
        UpdateSettingsRequest {
            id: "".to_string(),
            settings,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct UpgradeCanisterRequest {
    pub id: String,
    pub wasm_hash: WasmHash,
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
    pub deadline: u64,
}

impl UpgradeCanisterRequest {
    pub fn new(wasm: Wasm, wasm_version: WasmVersion) -> Self {
        UpgradeCanisterRequest {
            id: "".to_string(),
            wasm_version,
            wasm_hash: wasm.generate_hash(),
            wasm_hash_string: wasm.generate_hash_string(),
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct TopUpCanisterRequest {
    pub id: String,
    pub canister_id: CanisterId,
    pub amount: u64,
    pub deadline: u64,
}

impl TopUpCanisterRequest {
    pub fn new(canister_id: CanisterId, amount: u64) -> Self {
        TopUpCanisterRequest {
            id: "".to_string(),
            canister_id,
            amount,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct RawRandRequest {
    pub id: String,
    pub length: u32,
    pub deadline: u64,
}

impl RawRandRequest {
    pub fn new(length: u32) -> Self {
        RawRandRequest {
            id: "".to_string(),
            length,
            deadline: 0,
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CallRequest {
    pub id: String,
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
            id: "".to_string(),
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
    pub id: String,
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
            id: "".to_string(),
            canister_id,
            method_name,
            arg,
            sender,
            deadline,
        }
    }
}
