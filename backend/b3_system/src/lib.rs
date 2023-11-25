use b3_system_lib::{
    error::SystemError,
    store::{
        with_bugs, with_bugs_mut, with_canister_bugs_mut, with_hash_release, with_latest_release,
        with_release, with_release_mut, with_releases, with_releases_mut, with_state,
        with_state_mut, with_user_state, with_user_state_mut, with_users_mut, with_version_release,
        with_version_release_mut, with_wallet_canister,
    },
    types::{Canisters, LoadRelease, Release, ReleaseArgs, Releases, UserStates},
    user::UserState,
    wallet::WalletCanister,
};
use b3_utils::{
    caller_is_controller,
    constants::CREATE_WALLET_CANISTER_CYCLES,
    ic_canister_status,
    ledger::types::{Bug, SystemCanisterStatus, WalletCanisterInitArgs, WalletVersion},
    principal::StoredPrincipal,
    revert,
    types::{CanisterId, ControllerId, UserId},
    wasm::{Blob, WasmHash},
    NanoTimeStamp,
};
use ic_cdk::{api::management_canister::main::CanisterInstallMode, init, query, update};

#[init]
pub fn init() {}

#[query]
fn get_states() -> UserState {
    let user_id = ic_cdk::caller();

    with_state(|s| s.user_state(user_id.into())).unwrap_or_else(revert)
}

#[query]
fn get_create_canister_wallet_cycle() -> u128 {
    CREATE_WALLET_CANISTER_CYCLES
}

#[query(guard = "caller_is_controller")]
fn get_user_ids() -> Vec<UserId> {
    with_state(|s| s.user_ids())
}

#[query(guard = "caller_is_controller")]
fn get_user_states() -> UserStates {
    with_state(|s| s.user_states())
}

#[query]
fn get_canisters() -> Canisters {
    let user_id = ic_cdk::caller();

    with_user_state(user_id.into(), |s| s.canisters())
        .unwrap_or_else(revert)
        .unwrap_or_else(revert)
}

// UPDATE CALLS
#[update]
fn report_bug(bug: Bug) {
    let caller_id: StoredPrincipal = ic_cdk::caller().into();

    with_canister_bugs_mut(&caller_id, |bugs| bugs.push(bug)).unwrap_or_else(revert);
}

#[update]
fn clear_bugs() {
    with_bugs_mut(|bugs| {
        bugs.iter().for_each(|_| {
            bugs.clear();
        })
    });
}

#[query(guard = "caller_is_controller")]
fn get_bugs() -> Vec<Bug> {
    with_bugs(|bugs| bugs.iter().map(|b| b.clone()).collect())
}

#[update(guard = "caller_is_controller")]
async fn get_canister_version(canister_id: CanisterId) -> WalletVersion {
    let wallet = WalletCanister::new(canister_id);

    wallet.version().await.unwrap_or_else(revert)
}

#[update(guard = "caller_is_controller")]
async fn get_canister_version_by_user(user_id: UserId, index: usize) -> WalletVersion {
    let wallet = with_wallet_canister(user_id, index, |w| w.clone()).unwrap_or_else(revert);

    wallet.version().await.unwrap_or_else(revert)
}

#[update]
async fn create_wallet_canister() -> Result<UserState, String> {
    let system_id = ic_cdk::id();
    let caller_id = ic_cdk::caller();
    let owner_id: UserId = caller_id.into();

    let mut user_state = with_state_mut(|s| s.init_user(owner_id.clone())).unwrap_or_else(revert);

    let wallet_canister = user_state
        .create_with_cycles(vec![caller_id, system_id], CREATE_WALLET_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    with_state_mut(|s| s.add_user(owner_id.clone(), user_state.clone()));

    let init_args = WalletCanisterInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result =
        with_state_mut(|s| s.get_latest_install_args(CanisterInstallMode::Install, init_args));

    match install_arg_result {
        Ok(install_arg) => {
            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and add canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = wallet_canister
                .add_controllers(vec![caller_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user_state),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
async fn install_wallet_canister(canister_id: CanisterId) -> Result<UserState, String> {
    let system_id = ic_cdk::id();
    let caller_id = ic_cdk::caller();

    let owner_id: UserId = caller_id.into();

    let user_state = with_state_mut(|s| s.get_or_init_user(owner_id.clone(), Some(canister_id)))
        .unwrap_or_else(revert);

    let init_args = WalletCanisterInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result =
        with_state_mut(|s| s.get_latest_install_args(CanisterInstallMode::Install, init_args));

    match install_arg_result {
        Ok(install_arg) => {
            let wallet_canister = WalletCanister::new(canister_id);

            let status = wallet_canister.status().await;

            if status.is_ok() {
                revert(SystemError::WalletCanisterAlreadyInstalled)
            }

            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and add the user and canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = wallet_canister
                .add_controllers(vec![caller_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user_state),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
async fn add_wallet_canister(canister_id: CanisterId) {
    let user_id: UserId = ic_cdk::caller().into();

    let wallet_canister = WalletCanister::new(canister_id);

    let is_valid = wallet_canister
        .validate_signer(user_id.clone())
        .await
        .unwrap_or_else(revert);

    if is_valid {
        with_state_mut(|s| s.get_or_init_user(user_id, Some(canister_id))).unwrap_or_else(revert);
    } else {
        revert(SystemError::InvalidSigner)
    }
}

#[update]
fn change_wallet_canister(canister_id: CanisterId, index: usize) {
    let user_id: UserId = ic_cdk::caller().into();

    with_user_state_mut(&user_id, |s| s.change_canister(index, canister_id)).unwrap_or_else(revert);
}

#[update(guard = "caller_is_controller")]
fn remove_wallet_canister(user_id: UserId) {
    with_state_mut(|s| s.remove_user(&user_id));
}

// TODO: remove this method
#[update(guard = "caller_is_controller")]
fn reset_users() {
    with_users_mut(|s| s.clear());
}

#[query]
fn releases() -> Vec<Release> {
    with_releases(|r| r.iter().collect())
}

#[query]
fn latest_release() -> Release {
    with_latest_release(|r| r.clone()).unwrap_or_else(revert)
}

#[query]
pub fn get_release(version: WalletVersion) -> Release {
    with_version_release(version, |r| r.clone()).unwrap_or_else(revert)
}

#[query]
pub fn get_release_by_index(index: u64) -> Release {
    with_release(index, |r| r.clone()).unwrap_or_else(revert)
}

#[query]
pub fn get_release_by_hash_string(hash: WasmHash) -> Release {
    with_hash_release(hash, |r| r.clone()).unwrap_or_else(revert)
}

// UPDATE CALLS

#[update(guard = "caller_is_controller")]
fn update_release(name: String, release_args: ReleaseArgs) {
    let version = release_args.version.clone();

    with_version_release_mut(version, |vrs| {
        vrs.update(release_args);
    })
    .unwrap_or_else(revert)
}

#[update(guard = "caller_is_controller")]
fn load_release(blob: Blob, release_args: ReleaseArgs) -> LoadRelease {
    let version = release_args.version.clone();

    let release_index =
        with_releases_mut(|rs| match rs.iter().position(|r| r.version == version) {
            Some(index) => index,
            None => {
                let release = Release::new(release_args);
                rs.push(release);

                rs.len() - 1
            }
        });

    let total = with_release_mut(release_index, |r| r.load_wasm(&blob).unwrap_or_else(revert))
        .unwrap_or_else(revert);

    let chunks = blob.len();

    LoadRelease {
        version,
        chunks,
        total,
    }
}

#[update(guard = "caller_is_controller")]
pub fn remove_release(version: WalletVersion) -> Release {
    with_releases_mut(|rs| match rs.iter().position(|r| r.version == version) {
        Some(index) => Ok(rs.remove(index)),
        None => Err(SystemError::ReleaseNotFound),
    })
    .unwrap_or_else(revert)
}

#[update(guard = "caller_is_controller")]
fn remove_latest_release() {
    with_releases_mut(|rs| {
        rs.pop();
    })
}

#[update(guard = "caller_is_controller")]
fn deprecate_release(version: WalletVersion) {
    with_version_release_mut(version, |vrs| {
        vrs.deprecate();
    })
    .unwrap_or_else(revert)
}

#[query(guard = "caller_is_controller")]
fn get_controllers() -> Vec<ControllerId> {
    with_state(|s| s.get_controllers())
}

#[update(guard = "caller_is_controller")]
fn add_controller(controller_id: ControllerId) {
    with_state_mut(|s| {
        s.add_controller(controller_id);
    });
}

#[update(guard = "caller_is_controller")]
fn remove_controller(controller_id: ControllerId) {
    with_state_mut(|s| {
        s.remove_controller(controller_id);
    });
}

#[update(guard = "caller_is_controller")]
pub async fn status() -> SystemCanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = ic_canister_status(canister_id).await.unwrap_or_else(revert);

    let user_status = with_state(|s| s.number_of_users());
    let status_at = NanoTimeStamp::now();

    SystemCanisterStatus {
        canister_id,
        version,
        status_at,
        user_status,
        canister_status,
    }
}

#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

ic_cdk::export_candid!();
