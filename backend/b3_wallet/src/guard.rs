use b3_operations::{role::AccessLevel, store::with_verified_user};

pub fn caller_is_canister_or_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();

    with_verified_user(caller, |signer| {
        signer.have_access_level(&AccessLevel::FullAccess)
            || signer.have_access_level(&AccessLevel::Canister)
    })
}

pub fn caller_is_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();

    with_verified_user(caller, |signer| signer.is_admin())
}

pub fn _caller_is_user() -> Result<(), String> {
    let caller = ic_cdk::caller();

    with_verified_user(caller, |signer| signer.is_user())
}

pub fn caller_is_signer() -> Result<(), String> {
    let caller = ic_cdk::caller();

    with_verified_user(caller, |_| true)
}
