use b3_helper::types::Wasm;
use ic_cdk::api::management_canister::main::{CanisterInstallMode, InstallCodeArgument};

pub trait WasmTrait {
    fn upgrade_args(&self) -> InstallCodeArgument;
    fn reintall_args(&self) -> InstallCodeArgument;
}

impl WasmTrait for Wasm {
    fn upgrade_args(&self) -> InstallCodeArgument {
        let canister_id = ic_cdk::id();

        InstallCodeArgument {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
            wasm_module: self.get(),
            arg: Vec::new(),
        }
    }

    fn reintall_args(&self) -> InstallCodeArgument {
        let canister_id = ic_cdk::id();

        InstallCodeArgument {
            canister_id,
            mode: CanisterInstallMode::Reinstall,
            wasm_module: self.get(),
            arg: Vec::new(),
        }
    }
}
