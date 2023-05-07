use std::cell::RefCell;

use crate::guards::caller_is_owner;
use candid::candid_method;
use ic_cdk::{
    api::{
        call::CallResult,
        management_canister::main::{install_code, CanisterInstallMode, InstallCodeArgument},
    },
    query, update,
};

thread_local! {
    pub static WASM: RefCell<WasmData> = RefCell::new(WasmData::default());
}

pub struct WasmData {
    pub wasm: Vec<u8>,
    pub version: String,
}

impl Default for WasmData {
    fn default() -> Self {
        WasmData {
            wasm: Vec::new(),
            version: String::new(),
        }
    }
}

impl WasmData {
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

#[candid_method(query)]
#[query]
fn wasm_version() -> String {
    WASM.with(|s| s.borrow().version.clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn upgrade_canister() {
    let args = WASM.with(|s| s.borrow().upgrade_args().clone());

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn reintall_canister() {
    let args = WASM.with(|s| s.borrow().reintall_args().clone());

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn reset_wasm() {
    WASM.with(|s| s.borrow_mut().reset());
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn load_wasm(blob: Vec<u8>, version: String) -> CallResult<u64> {
    let mut wasm = WASM.with(|s| s.borrow().wasm.clone());

    wasm = wasm.iter().copied().chain(blob.iter().copied()).collect();

    let total = WASM.with(|s| {
        let state = &mut *s.borrow_mut();

        state.wasm = wasm;
        state.version = version;

        state.wasm.len()
    });

    Ok(total as u64)
}
