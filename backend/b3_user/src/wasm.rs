use ic_cdk::api::management_canister::main::{CanisterInstallMode, InstallCodeArgument};

pub struct Wasm {
    pub wasm: Vec<u8>,
    pub version: String,
}

impl Default for Wasm {
    fn default() -> Self {
        Wasm {
            wasm: Vec::new(),
            version: String::new(),
        }
    }
}

impl Wasm {
    pub fn upgrade_args(&self) -> InstallCodeArgument {
        let canister_id = ic_cdk::id();

        InstallCodeArgument {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
            wasm_module: self.wasm.clone(),
            arg: Vec::new(),
        }
    }

    pub fn reintall_args(&self) -> InstallCodeArgument {
        let canister_id = ic_cdk::id();

        InstallCodeArgument {
            canister_id,
            mode: CanisterInstallMode::Reinstall,
            wasm_module: self.wasm.clone(),
            arg: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.wasm.clear();
        self.version.clear();
    }
}
