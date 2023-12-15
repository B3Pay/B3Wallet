use b3_utils::{
    api::{AppInitArgs, AppInstallArg, AppVersion},
    memory::types::DefaultStableBTreeMap,
    wasm::{Wasm, WasmHash},
};
use ic_cdk::api::management_canister::main::CanisterInstallMode;

use super::release::Release;
use super::types::{AppId, AppReleaseArgs, CreateAppArgs};
use super::App;
use super::{error::AppSystemError, types::AppView};

pub type ReleaseMap = DefaultStableBTreeMap<WasmHash, Release>;
pub type AppMap = DefaultStableBTreeMap<AppId, App>;
pub type WasmMap = DefaultStableBTreeMap<AppVersion, Wasm>;

pub struct AppState {
    pub apps: AppMap,
    pub releases: ReleaseMap,
    pub wasm_map: WasmMap,
}

// Write to the AppState struct
impl AppState {
    pub fn add_app(&mut self, app_args: CreateAppArgs) -> Result<(), AppSystemError> {
        let app = App::new(app_args);

        let app_id = app.id();

        if self.apps.contains_key(&app_id) {
            return Err(AppSystemError::AppAlreadyExists);
        }

        self.apps.insert(app_id, app);

        Ok(())
    }

    pub fn update_app(&mut self, id: AppId, app_args: CreateAppArgs) -> Result<(), AppSystemError> {
        let app = self
            .apps
            .get(&id)
            .ok_or(AppSystemError::AppNotFound)?
            .update(app_args);

        self.apps.insert(id, app);

        Ok(())
    }

    pub fn remove_app(&mut self, id: AppId) -> Result<(), AppSystemError> {
        self.apps.remove(&id).ok_or(AppSystemError::AppNotFound)?;

        Ok(())
    }

    pub fn add_release(&mut self, wasm_hash: WasmHash, release_args: AppReleaseArgs) {
        let release = Release::new(release_args);

        self.releases.insert(wasm_hash, release);
    }

    pub fn update_release(&mut self, wasm_hash: WasmHash, release_args: AppReleaseArgs) {
        let release = Release::new(release_args);

        self.releases.insert(wasm_hash, release);
    }

    pub fn deprecate_release(&mut self, wasm_hash: WasmHash) -> Result<Release, AppSystemError> {
        let mut release = self
            .releases
            .get(&wasm_hash)
            .ok_or(AppSystemError::ReleaseNotFound)?;

        release.deprecate();

        self.releases.insert(wasm_hash, release.clone());

        Ok(release)
    }
}

// Read from the AppState struct
impl AppState {
    pub fn app_view(&self, id: AppId) -> Option<AppView> {
        self.apps.get(&id).map(|app| app.view())
    }

    pub fn app(&self, id: AppId) -> Option<App> {
        self.apps.get(&id)
    }

    pub fn release_view(&self, wasm_hash: &WasmHash) -> Option<super::types::ReleaseView> {
        self.releases.get(wasm_hash).map(|release| release.view())
    }

    pub fn release(&self, wasm_hash: &WasmHash) -> Result<Release, AppSystemError> {
        self.releases
            .get(wasm_hash)
            .ok_or(AppSystemError::ReleaseNotFound)
    }

    pub fn install_args(
        &self,
        wasm: Wasm,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, AppSystemError> {
        let arg = init_args
            .encode()
            .map_err(|e| AppSystemError::InstallArgError(e.to_string()))?;

        let wasm_module = wasm.bytes();

        Ok(AppInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn install_args_by_hash(
        &self,
        wasm_hash: &WasmHash,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, AppSystemError> {
        let wasm = self.release(wasm_hash)?.wasm()?;

        self.install_args(wasm, mode, init_args)
    }

    pub fn install_args_by_app(
        &self,
        id: AppId,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, AppSystemError> {
        let release = self
            .app(id)
            .ok_or(AppSystemError::AppNotFound)?
            .latest_release()
            .ok_or(AppSystemError::ReleaseNotFound)?;

        let wasm = release.wasm()?;

        self.install_args(wasm, mode, init_args)
    }
}
