use b3_system_lib::{
    app::{
        constants::CREATE_APP_CANISTER_CYCLES,
        error::AppSystemError,
        store::{
            with_app, with_app_mut, with_app_state, with_app_state_mut, with_release,
            with_release_mut, with_releases_mut, with_wasm_map,
        },
        types::{
            AppId, AppView, CreateAppArgs, CreateReleaseArgs, LoadRelease, ReleaseView,
            ReleaseViews,
        },
    },
    bug::store::{with_app_bugs, with_app_bugs_mut, with_bugs_mut},
    error::SystemError,
    types::UserCanisterStatus,
    user::{
        error::UserSystemError,
        store::{with_user, with_user_app, with_user_mut, with_user_state, with_user_state_mut},
        types::{UserStatus, UserView, UserViews},
    },
};
use b3_utils::{
    api::{
        bugs::{AppBug, AppBugs},
        AppCall, AppInitArgs, AppVersion, Management,
    },
    caller_is_controller,
    principal::StoredPrincipal,
    revert,
    types::{CanisterId, CanisterIds, UserId},
    wasm::{vec_to_wasm_hash, Blob, WasmHash, WasmVersion},
};
use candid::Principal;
use ic_cdk::{
    api::management_canister::main::{CanisterInfoResponse, CanisterInstallMode},
    init, query, update,
};

#[init]
pub fn init() {}

#[query]
fn get_states() -> UserView {
    let user_id = ic_cdk::caller().into();

    with_user(user_id, |s| s.view()).unwrap_or_else(revert)
}

#[query]
fn get_create_canister_app_cycle() -> u128 {
    CREATE_APP_CANISTER_CYCLES
}

#[query(guard = "caller_is_controller")]
fn get_user_ids() -> Vec<UserId> {
    with_user_state(|s| s.user_ids())
}

#[query(guard = "caller_is_controller")]
fn get_user_states() -> UserViews {
    with_user_state(|s| s.users_view())
}

#[query]
async fn get_user_status() -> UserStatus {
    let user_id: UserId = ic_cdk::caller().into();

    let canisters = with_user(user_id, |rs| rs.canisters());

    match canisters {
        Ok(canisters) => match canisters.len() {
            0 => UserStatus::Registered,
            1 => UserStatus::SingleCanister(canisters[0].clone()),
            _ => UserStatus::MultipleCanister(canisters),
        },
        Err(_) => UserStatus::Unregistered,
    }
}

#[query]
fn get_canisters() -> CanisterIds {
    let user_id = ic_cdk::caller();

    with_user(user_id.into(), |s| s.canisters()).unwrap_or_else(revert)
}

// UPDATE CALLS
#[update]
fn report_bug(bug: AppBug) {
    let caller_id: StoredPrincipal = ic_cdk::caller().into();

    with_app_bugs_mut(&caller_id, |bugs| bugs.push(bug)).unwrap_or_else(revert);
}

#[update(guard = "caller_is_controller")]
fn clear_bugs(canister_id: CanisterId) {
    let canister_id: StoredPrincipal = canister_id.into();

    with_bugs_mut(|bugs| bugs.remove(&canister_id));
}

#[query(guard = "caller_is_controller")]
fn get_bugs(canister_id: CanisterId) -> AppBugs {
    let canister_id: StoredPrincipal = canister_id.into();

    with_app_bugs(&canister_id, |bugs| bugs.clone()).unwrap_or_else(revert)
}

#[query(composite = true)]
async fn get_app_version(canister_id: CanisterId) -> AppVersion {
    let user_id: UserId = ic_cdk::caller().into();

    let app = with_user_app(user_id, &canister_id, |w| w.clone()).unwrap_or_else(revert);

    AppCall(app).version().await.unwrap_or_else(revert)
}

#[update]
async fn get_user_app_status(canister_id: CanisterId) -> UserCanisterStatus {
    let version = get_app_version(canister_id).await;

    let canister_status = Management::canister_status(canister_id)
        .await
        .unwrap_or_else(revert);

    UserCanisterStatus {
        version,
        canister_status,
    }
}

#[update]
async fn get_canister_info(canister_id: CanisterId) -> CanisterInfoResponse {
    Management::canister_info(canister_id, Some(10))
        .await
        .unwrap_or_else(revert)
}

#[update]
async fn create_app_canister(app_id: AppId) -> Result<UserView, String> {
    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();
    let user_id: UserId = owner_id.into();

    let mut user_state =
        with_user_state_mut(|s| s.initialize_user(user_id.clone())).unwrap_or_else(revert);

    let canister_id = user_state
        .create_with_cycles(vec![owner_id, system_id], CREATE_APP_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    with_user_state_mut(|s| s.add(user_id, user_state.clone()));

    let init_args = AppInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result = with_app_state(|s| {
        s.install_args_by_app_id(app_id, CanisterInstallMode::Install, init_args)
    });

    match install_arg_result {
        Ok(install_arg) => {
            // Install the code.
            let install_result = AppCall(canister_id).install_code(install_arg).await;

            // Update the controllers, and add canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = AppCall(canister_id)
                .add_controllers(vec![owner_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user_state.view()),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
async fn install_app(canister_id: CanisterId, app_id: AppId) -> Result<UserView, String> {
    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();

    let user_id: UserId = owner_id.into();

    let user_state = with_user_state_mut(|s| s.get_user_or_initialize(user_id, Some(canister_id)))
        .unwrap_or_else(revert);

    let init_args = AppInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result = with_app_state_mut(|s| {
        s.install_args_by_app_id(app_id, CanisterInstallMode::Install, init_args)
    });

    match install_arg_result {
        Ok(install_arg) => {
            let wallet_canister = AppCall(canister_id);

            let status = wallet_canister.status().await;

            if status.is_ok() {
                revert(SystemError::WalletCanisterAlreadyInstalled)
            }

            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and add the user and canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = wallet_canister
                .add_controllers(vec![owner_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user_state.view()),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
async fn add_user_app(canister_id: CanisterId, app_id: AppId) {
    let user_id: UserId = ic_cdk::caller().into();

    let wallet_canister = AppCall(canister_id);

    let module_hash = wallet_canister.module_hash().await.unwrap_or_else(revert);

    match module_hash {
        Some(module_hash) => {
            let wasm_hash = vec_to_wasm_hash(module_hash);
            let release = with_release(&wasm_hash, |r| r.clone()).unwrap_or_else(revert);

            if release.id() == app_id {
                let is_valid = wallet_canister
                    .validate_signer(user_id.clone())
                    .await
                    .unwrap_or_else(revert);

                if is_valid {
                    with_user_state_mut(|s| s.get_user_or_initialize(user_id, Some(canister_id)))
                        .unwrap_or_else(revert);
                } else {
                    revert(UserSystemError::InvalidUser)
                }
            } else {
                revert(AppSystemError::AppIdMismatch)
            }
        }
        None => revert(SystemError::WalletCanisterNotFound),
    }
}

#[update]
fn remove_user_app(canister_id: CanisterId) {
    let user_id: UserId = ic_cdk::caller().into();

    with_user_mut(&user_id, |rs| {
        rs.remove_canister(canister_id).unwrap_or_else(revert)
    })
    .unwrap_or_else(revert)
}

// TODO! Remove this update call for production.
#[update(guard = "caller_is_controller")]
fn remove_user(user_principal: Principal) {
    let user_id: UserId = user_principal.into();

    with_user_state_mut(|s| s.remove(&user_id));
}

#[update]
fn create_app(app_args: CreateAppArgs) -> AppView {
    let app = with_app_state_mut(|s| s.create_app(app_args)).unwrap_or_else(revert);

    app.view()
}

#[update]
fn update_app(app_id: AppId, app_args: CreateAppArgs) -> AppView {
    let app = with_app_state_mut(|s| s.update_app(app_id, app_args)).unwrap_or_else(revert);

    app.view()
}

#[query]
fn releases(app_id: AppId) -> ReleaseViews {
    with_app(&app_id, |app| app.release_views()).unwrap_or_else(revert)
}

#[query]
fn releases_wasm_hash() -> Vec<(WasmVersion, WasmHash)> {
    with_wasm_map(|wasm_map| {
        wasm_map
            .iter()
            .map(|(version, wasm)| (version.clone(), wasm.hash()))
            .collect()
    })
}

#[query]
fn get_latest_release(app_id: AppId) -> Option<ReleaseView> {
    with_app(&app_id, |app| app.latest_release_view()).unwrap_or_else(revert)
}

#[query]
pub fn get_release(wasm_hash: WasmHash) -> ReleaseView {
    with_release(&wasm_hash, |r| r.view()).unwrap_or_else(revert)
}

#[update(guard = "caller_is_controller")]
fn add_release(app_id: AppId, release_args: CreateReleaseArgs) {
    with_app_mut(&app_id, |app| app.add_release(release_args)).unwrap_or_else(revert);
}

#[update(guard = "caller_is_controller")]
fn load_release(wasm_hash: WasmHash, blob: Blob) -> LoadRelease {
    let total = with_release_mut(&wasm_hash, |rs| rs.load_wasm(&blob).unwrap_or_else(revert))
        .unwrap_or_else(revert);

    let chunks = blob.len();

    LoadRelease { chunks, total }
}

#[update(guard = "caller_is_controller")]
pub fn remove_release(wasm_hash: WasmHash) {
    with_releases_mut(|vrs| vrs.remove(&wasm_hash))
        .unwrap_or_else(|| revert(AppSystemError::ReleaseNotFound));
}

// #[update(guard = "caller_is_controller")]
// fn remove_latest_release() {
//     let latest_version = with_latest_version_release(|(version, _)| version).unwrap_or_else(revert);

//     remove_release(latest_version);
// }

// #[update(guard = "caller_is_controller")]
// fn deprecate_release(version: AppVersion) -> Release {
//     with_app_state_mut(|state| state.deprecate_release(version)).unwrap_or_else(revert)
// }

// #[update(guard = "caller_is_controller")]
// pub async fn status() -> SystemCanisterStatus {
//     let canister_id = ic_cdk::id();

//     let version = version();

//     let canister_status = Management::canister_status(canister_id)
//         .await
//         .unwrap_or_else(revert);

//     let user_status = with_state(|s| s.number_of_users());
//     let status_at = NanoTimeStamp::now();

//     SystemCanisterStatus {
//         version,
//         status_at,
//         user_status,
//         canister_status,
//     }
// }

#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

ic_cdk::export_candid!();
