use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;
