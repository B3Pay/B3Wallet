use b3_utils::{
    api::{AppInitArgs, AppInstallArg},
    wasm::WasmHash,
};
use ic_cdk::api::management_canister::main::CanisterInstallMode;

use crate::app::{
    error::AppSystemError,
    release::Release,
    store::{with_app, with_apps_mut},
    types::{AppId, AppView, CreateAppArgs, CreateReleaseArgs, ReleaseView},
    AppData,
};

pub struct WriteAppState(pub AppId);

impl WriteAppState {
    pub fn update(&mut self, app_args: CreateAppArgs) -> Result<AppData, AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.update(app_args);

            apps.insert(self.0.clone(), app.clone());

            Ok(app)
        })
    }

    pub fn increment_install_count(&mut self) -> Result<(), AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.increment_install_count();

            apps.insert(self.0.clone(), app.clone());

            Ok(())
        })
    }

    pub fn remove(&mut self) -> Result<(), AppSystemError> {
        self.remove_all_releases()?;

        with_apps_mut(|apps| {
            apps.remove(&self.0).ok_or(AppSystemError::AppNotFound)?;

            Ok(())
        })
    }

    pub fn add_release(
        &mut self,
        release_args: CreateReleaseArgs,
    ) -> Result<Release, AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            let release = app.add_release(release_args)?;

            apps.insert(self.0.clone(), app.clone());

            Ok(release)
        })
    }

    pub fn update_release(
        &mut self,
        release_args: CreateReleaseArgs,
    ) -> Result<(), AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.update_release(release_args);

            apps.insert(self.0.clone(), app.clone());

            Ok(())
        })
    }

    pub fn deprecate_release(&mut self, wasm_hash: WasmHash) -> Result<(), AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.deprecate_release(wasm_hash)?;

            apps.insert(self.0.clone(), app.clone());

            Ok(())
        })
    }

    pub fn remove_release(&mut self, wasm_hash: WasmHash) -> Result<(), AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.remove_release(wasm_hash)?;

            apps.insert(self.0.clone(), app.clone());

            Ok(())
        })
    }

    pub fn remove_all_releases(&mut self) -> Result<(), AppSystemError> {
        with_apps_mut(|apps| {
            let mut app = apps.get(&self.0).ok_or(AppSystemError::AppNotFound)?;

            app.remove_all_releases()?;

            apps.insert(self.0.clone(), app.clone());

            Ok(())
        })
    }
}

pub struct ReadAppState(pub AppId);

impl ReadAppState {
    pub fn validate(&self) -> Result<(), AppSystemError> {
        with_app(&self.0, |app| app.validate())?
    }

    pub fn app_view(&self) -> Result<AppView, AppSystemError> {
        with_app(&self.0, |app| app.view())
    }

    pub fn app(&self) -> Result<AppData, AppSystemError> {
        with_app(&self.0, |app| app.clone())
    }

    pub fn release_view(&self, wasm_hash: &WasmHash) -> Result<ReleaseView, AppSystemError> {
        with_app(&self.0, |app| app.release_view(wasm_hash))?
    }

    pub fn release(&self, wasm_hash: &WasmHash) -> Result<Release, AppSystemError> {
        with_app(&self.0, |app| app.release(wasm_hash))?
    }

    pub fn verify_release(&self, wasm_hash: &WasmHash) -> Result<(), AppSystemError> {
        with_app(&self.0, |app| app.verify_release(wasm_hash))?
    }

    pub fn latest_release(&self) -> Result<Release, AppSystemError> {
        with_app(&self.0, |app| app.latest_release())?
    }

    pub fn latest_release_view(&self) -> Result<ReleaseView, AppSystemError> {
        with_app(&self.0, |app| app.latest_release_view())?
    }

    pub fn release_views(&self) -> Result<Vec<ReleaseView>, AppSystemError> {
        with_app(&self.0, |app| app.release_views())
    }

    pub fn releases(&self) -> Result<Vec<Release>, AppSystemError> {
        with_app(&self.0, |app| app.releases())
    }

    pub fn install_args_by_wasm_hash(
        &self,
        wasm_hash: &WasmHash,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, AppSystemError> {
        let release = self.release(wasm_hash)?;

        let wasm_module = release.wasm_module()?;

        let arg = init_args
            .encode()
            .map_err(|e| AppSystemError::InstallArgError(e.to_string()))?;

        Ok(AppInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn install_args(
        &self,
        mode: CanisterInstallMode,
        init_args: AppInitArgs,
    ) -> Result<AppInstallArg, AppSystemError> {
        let release = self.latest_release()?;

        let wasm_module = release.wasm_module()?;

        let arg = init_args
            .encode()
            .map_err(|e| AppSystemError::InstallArgError(e.to_string()))?;

        Ok(AppInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }
}

pub struct AppState;

impl AppState {
    pub fn create(app_args: CreateAppArgs) -> Result<AppData, AppSystemError> {
        let app = AppData::new(app_args);
        let app_id = app.id();

        with_apps_mut(|apps| {
            if apps.contains_key(&app_id) {
                return Err(AppSystemError::AppAlreadyExists);
            }

            apps.insert(app_id, app.clone());

            Ok(app)
        })
    }

    pub fn write(app_id: AppId) -> WriteAppState {
        WriteAppState(app_id)
    }

    pub fn read(app_id: AppId) -> ReadAppState {
        ReadAppState(app_id)
    }
}
