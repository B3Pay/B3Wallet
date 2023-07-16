use crate::error::WalletError;
use b3_helper_lib::{
    ic_canister_status,
    types::{ControllerId, ControllerIds, Metadata, WalletController, WalletControllerMap},
};
use ic_cdk::api::management_canister::{
    main::{update_settings, UpdateSettingsArgument},
    provisional::CanisterSettings,
};
use ic_cdk::export::{
    candid::{CandidType, Nat},
    serde::Deserialize,
};

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletSettings {
    pub metadata: Metadata,
    pub controllers: WalletControllerMap,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
    pub initialised: bool,
}

impl Default for WalletSettings {
    fn default() -> Self {
        WalletSettings {
            metadata: Metadata::default(),
            controllers: WalletControllerMap::default(),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            initialised: false,
        }
    }
}

impl WalletSettings {
    pub fn new(controllers: WalletControllerMap, metadata: Option<Metadata>) -> Self {
        WalletSettings {
            controllers,
            metadata: metadata.unwrap_or_default(),
            ..Default::default()
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn controllers(&self) -> &WalletControllerMap {
        &self.controllers
    }

    pub fn controllers_mut(&mut self) -> &mut WalletControllerMap {
        &mut self.controllers
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    pub async fn refresh_settings(&mut self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let canister_status = ic_canister_status(canister_id)
            .await
            .map_err(WalletError::HelperError)?;

        let settings = canister_status.settings;

        self.update_controllers(settings.controllers);

        self.compute_allocation = Some(settings.compute_allocation);
        self.memory_allocation = Some(settings.memory_allocation);
        self.freezing_threshold = Some(settings.freezing_threshold);

        Ok(())
    }

    pub async fn update_controller_and_update(
        &mut self,
        canister_map: WalletControllerMap,
    ) -> Result<(), WalletError> {
        if canister_map.len() > 10 {
            return Err(WalletError::TooManyControllers);
        }

        self.controllers = canister_map;

        self.update_settings().await?;

        Ok(())
    }

    /// Update the controllers of the canister.
    /// The canister itself is always a controller.
    /// The canister can have up to 10 controllers.
    /// returns the updated controllers.
    pub async fn add_controller_and_update(
        &mut self,
        controller_id: ControllerId,
        controller: WalletController,
    ) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let mut controller_ids = vec![controller_id, canister_id];

        let canister_status = ic_canister_status(canister_id)
            .await
            .map_err(WalletError::HelperError)?;

        let settings = canister_status.settings;

        settings.controllers.iter().for_each(|controller| {
            if !controller_ids.contains(controller) {
                controller_ids.push(controller.clone());
            }
        });

        if controller_ids.len() > 10 {
            return Err(WalletError::TooManyControllers);
        }

        let arg = UpdateSettingsArgument {
            canister_id,
            settings: CanisterSettings {
                controllers: Some(controller_ids.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            },
        };

        update_settings(arg)
            .await
            .map_err(|err| WalletError::UpdateCanisterControllersError(err.1))?;

        self.controllers.insert(controller_id, controller);

        // check the controller is exist in self.controllers if not add it with unknown name
        self.update_controllers(controller_ids);

        Ok(())
    }

    /// Update the controllers of the canister.
    /// The canister itself is always a controller.
    /// The maximum number of controllers is 10.
    pub async fn update_settings(&mut self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let mut controller_ids = self.controllers.keys().cloned().collect::<Vec<_>>();

        if !controller_ids.contains(&canister_id) {
            controller_ids.push(canister_id);
        }

        if controller_ids.len() > 10 {
            return Err(WalletError::TooManyControllers);
        }

        let arg = UpdateSettingsArgument {
            canister_id,
            settings: CanisterSettings {
                controllers: Some(controller_ids.clone()),
                compute_allocation: self.compute_allocation.clone(),
                memory_allocation: self.memory_allocation.clone(),
                freezing_threshold: self.freezing_threshold.clone(),
            },
        };

        update_settings(arg)
            .await
            .map_err(|err| WalletError::UpdateCanisterControllersError(err.1))?;

        self.update_controllers(controller_ids);

        Ok(())
    }

    fn update_controllers(&mut self, controller_ids: ControllerIds) {
        let canister_id = ic_cdk::id();

        controller_ids
            .iter()
            .fold(WalletControllerMap::new(), |mut acc, id| {
                if let Some(controller) = self.controllers.get(id) {
                    acc.insert(id.clone(), controller.clone());
                } else {
                    let name = if id == &canister_id {
                        "self"
                    } else {
                        "unknown"
                    };

                    let controller = WalletController::new(name.to_owned(), None);
                    acc.insert(id.clone(), controller);
                }
                acc
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use b3_helper_lib::types::ControllerIds;
    use ic_cdk::export::Principal;

    #[test]
    fn test_remove_controller() {
        let mut controller_map = WalletControllerMap::default();

        for i in 0..4u8 {
            let canister_id = Principal::from_slice(&[i; 29]);

            let controller = WalletController::new(format!("controller-{}", i), None);

            controller_map.insert(canister_id, controller);
        }

        let canister_id = Principal::from_slice(&[4; 29]);

        let controller = WalletController::new(format!("controller-{}", 4), None);

        controller_map.insert(canister_id, controller);

        assert_eq!(controller_map.len(), 5);

        let mut controllers = ControllerIds::new();

        for i in 0..6u8 {
            let canister_id = Principal::from_slice(&[i; 29]);

            controllers.push(canister_id);
        }

        controller_map = controllers
            .iter()
            .fold(WalletControllerMap::new(), |mut acc, id| {
                if let Some(controller) = controller_map.get(id) {
                    acc.insert(id.clone(), controller.clone());
                } else {
                    let controller = WalletController::new("unknown".to_owned(), None);
                    acc.insert(id.clone(), controller);
                }
                acc
            });

        println!("{:?}", controller_map); // prints: [1, 3, 5]
    }
}
