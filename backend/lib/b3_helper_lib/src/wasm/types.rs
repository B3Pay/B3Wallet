use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

pub type WasmSize = usize;
pub type WasmModule = Vec<u8>;
pub type WasmHash = [u8; 32];
pub type WasmHashString = String;
pub type WasmVersion = String;
pub type Blob = Vec<u8>;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct WasmDetails {
    pub hash: WasmHash,
    pub size: WasmSize,
}
