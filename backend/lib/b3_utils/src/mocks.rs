use candid::Principal;

pub fn time_mock() -> u64 {
    use std::time::SystemTime;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

//only use for test cases
pub fn id_mock() -> Principal {
    Principal::management_canister()
}
