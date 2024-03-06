use std::collections::HashMap;

use crate::error::WalletError;
use b3_utils::{
    api::Management,
    ledger::{Metadata, Value},
    types::{AppControllerMap, ControllerId, ControllerIds},
};
use candid::{CandidType, Nat};
use ic_cdk::api::management_canister::{
    main::UpdateSettingsArgument, provisional::CanisterSettings,
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct WalletSettings {
    pub metadata: Metadata,
    pub controllers: HashMap<ControllerId, String>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
    pub reserved_cycles_limit: Option<Nat>,
    pub initialised: bool,
}

impl Default for WalletSettings {
    fn default() -> Self {
        WalletSettings {
            metadata: Metadata::default(),
            controllers: HashMap::new(),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            initialised: false,
        }
    }
}

impl WalletSettings {
    pub fn new(controllers: AppControllerMap, metadata: Option<Metadata>) -> Self {
        WalletSettings {
            controllers,
            metadata: metadata.unwrap_or_default(),
            ..Default::default()
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn controllers(&self) -> &AppControllerMap {
        &self.controllers
    }

    pub fn controllers_mut(&mut self) -> &mut AppControllerMap {
        &mut self.controllers
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn add_metadata(&mut self, key: String, value: Value) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    pub async fn refresh_settings(&mut self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let canister_status = Management::canister_status(canister_id.clone())
            .await
            .map_err(WalletError::ManagmentError)?;

        let settings = canister_status.settings;

        self.update_controllers(settings.controllers);

        self.compute_allocation = Some(settings.compute_allocation);
        self.memory_allocation = Some(settings.memory_allocation);
        self.freezing_threshold = Some(settings.freezing_threshold);

        Ok(())
    }

    pub async fn update_controller_and_update(
        &mut self,
        canister_map: AppControllerMap,
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
        name: String,
    ) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let mut controller_ids = vec![controller_id, canister_id];

        let canister_status = Management::canister_status(canister_id)
            .await
            .map_err(WalletError::ManagmentError)?;

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
                ..Default::default()
            },
        };

        Management::update_settings(arg)
            .await
            .map_err(|err| WalletError::UpdateCanisterControllersError(err.to_string()))?;

        self.controllers.insert(controller_id, name);

        // check the controller is exist in self.controllers if not add it with unknown name
        self.update_controllers(controller_ids);

        Ok(())
    }

    /// Update the controllers of the canister.
    /// The canister itself is always a controller.
    /// The maximum number of controllers is 10.
    pub async fn update_settings(&mut self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let mut controller_ids: ControllerIds =
            self.controllers.iter().map(|(id, _)| id.clone()).collect();

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
                reserved_cycles_limit: self.reserved_cycles_limit.clone(),
            },
        };

        Management::update_settings(arg)
            .await
            .map_err(|err| WalletError::UpdateCanisterControllersError(err.to_string()))?;

        self.update_controllers(controller_ids);

        Ok(())
    }

    fn update_controllers(&mut self, controller_ids: ControllerIds) {
        let canister_id = ic_cdk::id();

        controller_ids
            .iter()
            .fold(AppControllerMap::new(), |mut acc, id| {
                if let Some(name) = self.controllers.get(id) {
                    acc.insert(id.clone(), name.clone());
                } else {
                    let name = if id == &canister_id {
                        "self"
                    } else {
                        "unknown"
                    };

                    acc.insert(id.clone(), name.to_owned());
                }
                acc
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use b3_utils::types::ControllerIds;
    use candid::Principal;

    #[test]
    fn test_remove_controller() {
        let mut controller_map = AppControllerMap::new();

        for i in 0..4u8 {
            let canister_id = Principal::from_slice(&[i; 29]);
            let name = format!("controller-{}", i);

            controller_map.insert(canister_id, name);
        }

        let canister_id = Principal::from_slice(&[4; 29]);

        let name = format!("controller-{}", 4);

        controller_map.insert(canister_id, name);

        assert_eq!(controller_map.len(), 5);

        let mut controllers = ControllerIds::new();

        for i in 0..6u8 {
            let canister_id = Principal::from_slice(&[i; 29]);

            controllers.push(canister_id);
        }

        controller_map = controllers
            .iter()
            .fold(AppControllerMap::new(), |mut acc, id| {
                if let Some(controller) = controller_map.get(id) {
                    acc.insert(id.clone(), controller.clone());
                } else {
                    let name = "unknown".to_owned();
                    acc.insert(id.clone(), name);
                }
                acc
            });

        println!("{:?}", controller_map); // prints: [1, 3, 5]
    }
}
