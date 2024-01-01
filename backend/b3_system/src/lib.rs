use b3_system_lib::{
    app::{
        constants::CREATE_APP_CANISTER_CYCLES,
        error::AppSystemError,
        state::AppState,
        store::{with_release, with_release_mut, with_releases_mut},
        types::{
            AppId, AppView, CreateAppArgs, CreateReleaseArgs, LoadRelease, ReleaseView,
            ReleaseViews, SystemCanisterStatus,
        },
    },
    bug::store::{with_app_bugs, with_app_bugs_mut, with_bugs_mut},
    error::SystemError,
    types::UserCanisterStatus,
    user::{
        error::UserSystemError,
        store::with_users,
        types::{CreateUserArgs, UserStatus, UserView, UserViews},
        UserState,
    },
};
use b3_utils::{
    api::{
        bugs::{AppBug, AppBugs},
        AppCall, AppInitArgs, AppVersion, Management,
    },
    caller_is_controller, hex_string_with_0x_to_vec,
    principal::StoredPrincipal,
    revert,
    types::{CanisterId, CanisterIds, UserId},
    wasm::{vec_to_wasm_hash, Blob, WasmHash},
    NanoTimeStamp,
};
use candid::Principal;
use ic_cdk::{
    api::management_canister::main::{CanisterInfoResponse, CanisterInstallMode},
    init, query, update,
};

#[init]
fn init() {}

#[query]
fn get_states() -> UserView {
    let user_id = ic_cdk::caller().into();

    UserState::read(user_id).user_view().unwrap_or_else(revert)
}

#[query]
fn get_create_canister_app_cycle() -> u128 {
    CREATE_APP_CANISTER_CYCLES
}

#[query(guard = "caller_is_controller")]
fn get_user_ids() -> Vec<UserId> {
    with_users(|s| s.iter().map(|(k, _)| k.clone()).collect())
}

#[query(guard = "caller_is_controller")]
fn get_user_states() -> UserViews {
    with_users(|s| s.iter().map(|(_, v)| v.view()).collect())
}

#[query]
async fn get_user_status() -> UserStatus {
    let user = UserState::read(ic_cdk::caller().into());

    match user.canisters() {
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
    let user = UserState::read(ic_cdk::caller().into());

    user.canisters().unwrap_or_else(revert)
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
    let user = UserState::read(ic_cdk::caller().into());

    user.verify_canister(&canister_id).unwrap_or_else(revert);

    AppCall(canister_id).version().await.unwrap_or_else(revert)
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
async fn create_user(app_args: CreateUserArgs) -> UserView {
    let user = UserState::create(app_args).unwrap_or_else(revert);

    user.view()
}

#[update]
async fn create_app_canister(app_id: AppId) -> Result<UserView, String> {
    AppState::read(app_id.clone())
        .validate()
        .unwrap_or_else(revert);

    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();

    let user_id: UserId = owner_id.into();

    let mut user = UserState::read(user_id.clone())
        .user()
        .unwrap_or_else(revert);

    let canister_id = user
        .create_with_cycles(vec![owner_id, system_id], CREATE_APP_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    UserState::write(user_id)
        .add_canister(canister_id.clone())
        .unwrap_or_else(revert);

    let init_args = AppInitArgs {
        owner_id,
        system_id,
    };

    match AppState::read(app_id).install_args(CanisterInstallMode::Install, init_args) {
        Ok(install_arg) => {
            // Install the code.
            let install_result = AppCall(canister_id).install_code(install_arg).await;

            // Update the controllers, and add canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = AppCall(canister_id)
                .add_controllers(vec![owner_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user.view()),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
async fn install_app(canister_id: CanisterId, app_id: AppId) -> Result<UserView, String> {
    AppState::read(app_id.clone())
        .validate()
        .unwrap_or_else(revert);

    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();

    let user_id: UserId = owner_id.into();

    let user_view = UserState::read(user_id).user_view().unwrap_or_else(revert);

    let init_args = AppInitArgs {
        owner_id,
        system_id,
    };

    match AppState::read(app_id).install_args(CanisterInstallMode::Upgrade, init_args) {
        Ok(install_arg) => {
            let app_call = AppCall(canister_id);

            let status = app_call.status().await;

            if status.is_ok() {
                revert(SystemError::AppCanisterAlreadyInstalled)
            }

            // Install the code.
            let install_result = app_call.install_code(install_arg).await;

            // Update the controllers, and add the user and canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = app_call.add_controllers(vec![owner_id, system_id]).await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(user_view),
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

    UserState::read(user_id.clone())
        .user()
        .unwrap_or_else(revert);

    let app_call = AppCall(canister_id);

    let module_hash = app_call.module_hash().await.unwrap_or_else(revert);

    match module_hash {
        Some(module_hash) => {
            let wasm_hash = vec_to_wasm_hash(module_hash);

            AppState::read(app_id.clone())
                .verify_release(&wasm_hash)
                .unwrap_or_else(revert);

            let is_valid = app_call
                .validate_user(user_id.clone())
                .await
                .unwrap_or_else(revert);

            if is_valid {
                UserState::write(user_id)
                    .add_canister(canister_id)
                    .unwrap_or_else(revert);
            } else {
                revert(UserSystemError::InvalidUser)
            }
        }
        None => revert(SystemError::AppCanisterNotFound),
    }
}

#[update]
fn remove_user_app(canister_id: CanisterId) {
    let user_id: UserId = ic_cdk::caller().into();

    UserState::write(user_id)
        .remove_canister(canister_id)
        .unwrap_or_else(revert);
}

// TODO! Remove this update call for production.
#[update(guard = "caller_is_controller")]
fn remove_user(user_principal: Principal) {
    let user_id: UserId = user_principal.into();

    UserState::write(user_id).remove().unwrap_or_else(revert);
}

#[update]
fn create_app(app_args: CreateAppArgs) -> Result<AppView, String> {
    match AppState::create(app_args) {
        Ok(app) => Ok(app.view()),
        Err(err) => Err(err.to_string()),
    }
}

#[update]
fn get_app(app_id: AppId) -> Result<AppView, String> {
    match AppState::read(app_id).app_view() {
        Ok(app) => Ok(app),
        Err(err) => Err(err.to_string()),
    }
}

#[update]
fn update_app(app_id: AppId, app_args: CreateAppArgs) -> Result<AppView, String> {
    match AppState::write(app_id).update(app_args) {
        Ok(app) => Ok(app.view()),
        Err(err) => Err(err.to_string()),
    }
}

#[query]
fn releases(app_id: AppId) -> ReleaseViews {
    AppState::read(app_id)
        .release_views()
        .unwrap_or_else(revert)
}

#[query]
fn get_latest_release(app_id: AppId) -> Result<ReleaseView, String> {
    AppState::read(app_id)
        .latest_release_view()
        .map_err(|e| e.to_string())
}

#[query]
fn get_release(wasm_hash: WasmHash) -> ReleaseView {
    with_release(&wasm_hash, |release| release.view()).unwrap_or_else(revert)
}

#[query]
fn get_release_by_hash_string(wasm_hash_string: String) -> ReleaseView {
    let hash = hex_string_with_0x_to_vec(wasm_hash_string).unwrap();

    let wasm_hash = vec_to_wasm_hash(hash);

    with_release(&wasm_hash, |release| release.view()).unwrap_or_else(revert)
}

#[update(guard = "caller_is_controller")]
fn add_release(app_id: AppId, release_args: CreateReleaseArgs) -> ReleaseView {
    let release = AppState::write(app_id)
        .add_release(release_args)
        .unwrap_or_else(revert);

    release.view()
}

#[update(guard = "caller_is_controller")]
fn load_wasm_chunk(wasm_hash: WasmHash, chunk: Blob) -> LoadRelease {
    let total = with_release_mut(&wasm_hash, |rs| {
        rs.load_wasm_chunk(&chunk).unwrap_or_else(revert)
    })
    .unwrap_or_else(revert);

    let chunks = chunk.len();

    LoadRelease { chunks, total }
}

#[update(guard = "caller_is_controller")]
fn remove_release(wasm_hash: WasmHash) {
    with_releases_mut(|vrs| vrs.remove(&wasm_hash))
        .unwrap_or_else(|| revert(AppSystemError::ReleaseNotFound));
}

#[update(guard = "caller_is_controller")]
fn deprecate_release(app_id: AppId, wasm_hash: WasmHash) -> ReleaseView {
    AppState::write(app_id)
        .deprecate_release(wasm_hash)
        .unwrap_or_else(revert)
        .view()
}

#[update(guard = "caller_is_controller")]
async fn status() -> SystemCanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = Management::canister_status(canister_id)
        .await
        .unwrap_or_else(revert);

    let user_status = with_users(|s| s.len());
    let status_at = NanoTimeStamp::now();

    SystemCanisterStatus {
        version,
        status_at,
        user_status,
        canister_status,
    }
}

#[query]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

ic_cdk::export_candid!();
