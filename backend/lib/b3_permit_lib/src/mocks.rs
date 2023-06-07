use candid::Principal;

pub fn ic_timestamp() -> u64 {
    u64::from(1667817318 as u64)
}

//only use for test cases
pub fn ic_cdk_id() -> Principal {
    Principal::management_canister()
}
