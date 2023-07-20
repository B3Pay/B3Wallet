use candid::Principal;

pub fn time_mock() -> u64 {
    u64::from(1689671609000000000 as u64)
}

//only use for test cases
pub fn id_mock() -> Principal {
    Principal::management_canister()
}
