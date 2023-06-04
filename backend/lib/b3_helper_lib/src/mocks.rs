use candid::Principal;

pub fn ic_timestamp() -> u64 {
    u64::from(1667817318 as u64)
}

pub fn ic_cdk_id() -> Principal {
    Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap()
}
