use crate::{
    error::SystemError,
    types::ReleaseArgs,
    types::{AppId, ReleaseVersion},
};
use b3_utils::{
    api::{AppInitArgs, AppInstallArg, AppVersion},
    memory::types::DefaultStableBTreeMap,
    wasm::{Wasm, WasmVersion},
};
use ic_cdk::api::management_canister::main::CanisterInstallMode;

pub mod app;
pub mod constants;
pub mod release;
pub mod store;

use app::App;
use release::Release;

pub type ReleaseMap = DefaultStableBTreeMap<ReleaseVersion, Release>;
pub type AppMap = DefaultStableBTreeMap<AppId, App>;
pub type WasmMap = DefaultStableBTreeMap<AppVersion, Wasm>;

pub struct AppState {
    pub apps: AppMap,
    pub releases: ReleaseMap,
    pub wasm_map: WasmMap,
}

impl AppState {
    // App
    pub fn add_app(&mut self, product: App) -> Result<(), SystemError> {
        if self.apps.contains_key(&product.id) {
            return Err(SystemError::ProductAlreadyExists); // Assuming you define this error
        }
        self.apps.insert(product.id.clone(), product);
        Ok(())
    }

    pub fn get_app(&self, id: AppId) -> Option<App> {
        self.apps.get(&id)
    }

    // release
    pub fn get_release(&self, version: &ReleaseVersion) -> Result<Release, SystemError> {
        self.releases
            .get(version)
            .ok_or(SystemError::ReleaseNotFound)
    }

    pub fn get_release_install_args(
        &self,
        version: &ReleaseVersion,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, SystemError> {
        let wasm_module = self.get_release(version)?.wasm()?.bytes();

        let arg = init_args
            .encode()
            .map_err(|e| SystemError::InstallArgError(e.to_string()))?;

        Ok(AppInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn latest_release(&self) -> Result<Release, SystemError> {
        self.releases
            .last_key_value()
            .ok_or(SystemError::ReleaseNotFound)
            .map(|(_, release)| release)
    }

    pub fn get_latest_install_args(
        &self,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, SystemError> {
        let wasm_module = self.latest_release()?.wasm()?.bytes();

        let arg = init_args
            .encode()
            .map_err(|e| SystemError::InstallArgError(e.to_string()))?;

        Ok(AppInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn update_release(&mut self, release: ReleaseArgs) {
        let version = release.version.clone();

        self.releases.insert(version, release.into());
    }

    pub fn deprecate_release(&mut self, version: ReleaseVersion) -> Result<Release, SystemError> {
        let mut release = self
            .releases
            .get(&version)
            .ok_or(SystemError::ReleaseNotFound)?;

        release.deprecate();

        self.releases.insert(version, release.clone());

        Ok(release)
    }

    pub fn add_feature_release(
        &mut self,
        version: WasmVersion,
        feature: String,
    ) -> Result<Release, SystemError> {
        let mut release = self
            .releases
            .get(&version)
            .ok_or(SystemError::ReleaseNotFound)?;

        release.add_feature(feature);

        self.releases.insert(version, release.clone());

        Ok(release)
    }

    pub fn remove_feature_release(
        &mut self,
        version: WasmVersion,
        feature: String,
    ) -> Result<Release, SystemError> {
        let mut release = self
            .releases
            .get(&version)
            .ok_or(SystemError::ReleaseNotFound)?;

        release.remove_feature(feature);

        self.releases.insert(version, release.clone());

        Ok(release)
    }
}
