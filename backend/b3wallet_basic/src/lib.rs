use b3_utils::{
    api::{bugs::AppBug, AppAccountsNonce, AppInitArgs, AppStatus, Management},
    ledger::currency::{ICPToken, TokenAmount},
    ledger::{Metadata, NotifyTopUpResult, TransferBlockIndex, Value},
    log_cycle,
    logs::{export_log, export_log_messages_page, LogEntry},
    owner::caller_is_owner,
    panic_log,
    types::{AppControllerMap, CanisterId, ControllerId},
    wasm::{with_wasm_cache, with_wasm_mut_cache, WasmDetails, WasmHash, WasmSize},
    Environment, NanoTimeStamp, Subaccount,
};
use b3wallet_lib::{
    account::WalletAccount,
    error::WalletError,
    ledger::{
        btc::{network::BitcoinNetwork, types::UtxoStatus},
        chain::ChainTrait,
        ckbtc::{minter::Minter, types::RetrieveBtcStatus},
        subaccount::SubaccountEcdsaTrait,
        types::{AddressMap, Balance, BtcPending, ChainEnum, PendingEnum, SendResult},
    },
    setting::WalletSettings,
    store::{
        with_account, with_account_mut, with_chain, with_chain_mut, with_ledger, with_ledger_mut,
        with_setting, with_setting_mut, with_wallet, with_wallet_mut,
    },
    types::{AccountId, WalletAccountView},
};
use candid::{CandidType, Deserialize};
use ic_cdk::{
    api::{
        call::arg_data,
        management_canister::{
            bitcoin::Satoshi,
            main::{install_code, uninstall_code, CanisterInstallMode, InstallCodeArgument},
            provisional::CanisterIdRecord,
        },
    },
    init, post_upgrade, pre_upgrade, query, update,
};

#[init]
fn init() {
    log_cycle!("init");
    // when the canister is created by another canister (e.g. the system canister)
    // this function is called with the arguments passed to the canister constructor.
    let (call_arg,) = arg_data::<(Option<AppInitArgs>,)>();

    match call_arg {
        Some(args) => {
            let AppInitArgs {
                owner_id,
                system_id,
            } = args;
            with_setting_mut(|s| {
                s.controllers.insert(ic_cdk::id(), "Self".to_owned());
                s.controllers.insert(system_id, "System".to_owned());
                s.controllers.insert(owner_id, "Owner".to_owned());
            });
        }
        None => {}
    };
}

#[pre_upgrade]
fn pre_upgrade() {
    log_cycle!("pre_upgrade");
    with_wasm_mut_cache(|wasm| wasm.unload());
}

#[post_upgrade]
fn post_upgrade() {
    log_cycle!("post_upgrade");
}

#[query(guard = "caller_is_owner")]
fn get_account(account_id: AccountId) -> WalletAccountView {
    with_account(&account_id, |account| account.view()).unwrap_or_else(panic_log)
}

#[query(guard = "caller_is_owner")]
fn get_account_count() -> usize {
    with_wallet(|s| s.accounts_len())
}

#[query(guard = "caller_is_owner")]
fn get_account_counters() -> AppAccountsNonce {
    with_wallet(|s| s.counters().clone())
}

#[query(guard = "caller_is_owner")]
fn get_account_views() -> Vec<WalletAccountView> {
    with_wallet(|s| s.account_views())
}

#[query(guard = "caller_is_owner")]
fn get_account_view(account_id: AccountId) -> WalletAccountView {
    with_account(&account_id, |account| account.view()).unwrap_or_else(panic_log)
}

#[query(guard = "caller_is_owner")]
fn get_addresses(account_id: AccountId) -> AddressMap {
    with_ledger(&account_id, |ledger| ledger.address_map().clone()).unwrap_or_else(panic_log)
}

#[query(guard = "caller_is_owner")]
async fn retrieve_btc_status(
    network: BitcoinNetwork,
    block_index: TransferBlockIndex,
) -> RetrieveBtcStatus {
    let minter = Minter(network);

    minter
        .retrieve_btc_status(block_index)
        .await
        .unwrap_or_else(panic_log)
}

// UPDATE ---------------------------------------------------------------------
#[update(guard = "caller_is_owner")]
async fn account_update_balance(account_id: AccountId, network: BitcoinNetwork) -> Vec<UtxoStatus> {
    log_cycle!(
        "Update balance for account: {} on network: {}",
        account_id,
        network
    );

    let ckbtc = with_chain(&account_id, &ChainEnum::CKBTC(network), |chain| {
        chain.ckbtc()
    })
    .unwrap_or_else(panic_log)
    .unwrap_or_else(panic_log);

    let reuslt = ckbtc.update_balance().await.unwrap_or_else(panic_log);

    match reuslt {
        Ok(utxos) => utxos,
        Err(err) => panic_log(err),
    }
}

#[derive(CandidType, Deserialize)]
struct AccountCreateArgs {
    env: Option<Environment>,
    name: Option<String>,
}

#[update(guard = "caller_is_owner")]
fn account_create(AccountCreateArgs { env, name }: AccountCreateArgs) {
    log_cycle!("Create account: {:?} on env: {:?}", name, env);

    let subaccount = with_wallet(|s| s.new_subaccount(env));

    let new_account = WalletAccount::from(subaccount);

    with_wallet_mut(|s| s.insert_account(new_account, name));
}

#[update(guard = "caller_is_owner")]
fn account_rename(account_id: AccountId, name: String) {
    log_cycle!("Rename account: {} to {}", account_id, name);

    with_account_mut(&account_id, |a| a.rename(name)).unwrap_or_else(panic_log)
}

#[update(guard = "caller_is_owner")]
fn account_hide(account_id: AccountId) {
    log_cycle!("Hide account: {}", account_id);

    with_account_mut(&account_id, |a| a.hide()).unwrap_or_else(panic_log)
}

#[update(guard = "caller_is_owner")]
fn account_remove(account_id: AccountId) {
    log_cycle!("Remove account: {}", account_id);

    with_wallet_mut(|s| s.remove_account(&account_id)).unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
fn account_remove_address(account_id: AccountId, chain: ChainEnum) {
    log_cycle!("Remove address: {} on chain: {:?}", account_id, chain);

    with_ledger_mut(&account_id, |ledger| ledger.remove_address(chain))
        .unwrap_or_else(panic_log)
        .unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
fn account_restore(env: Environment, nonce: u64) {
    log_cycle!("Restore account: {:?} with nonce: {}", env, nonce);

    let subaccount = Subaccount::new(env, nonce);

    with_wallet_mut(|s| s.restore_account(subaccount)).unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
async fn account_balance(account_id: AccountId, chain: ChainEnum) -> Balance {
    log_cycle!(
        "Get balance for account: {} on chain: {:?}",
        account_id,
        chain
    );

    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(panic_log);

    let balance = ledger.balance(chain).await;

    match balance {
        Ok(balance) => balance,
        Err(err) => panic_log(err),
    }
}

#[update(guard = "caller_is_owner")]
async fn account_send(
    account_id: AccountId,
    chain: ChainEnum,
    to: String,
    amount: TokenAmount,
) -> SendResult {
    log_cycle!(
        "Send {} on chain: {:?} from account: {} to: {}",
        amount,
        chain,
        account_id,
        to
    );

    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(panic_log);

    ledger
        .send(&chain, to, amount)
        .await
        .unwrap_or_else(panic_log)
}

#[update(guard = "caller_is_owner")]
async fn account_check_pending(account_id: AccountId, chain_enum: ChainEnum, pending_index: usize) {
    log_cycle!(
        "Check pending: {} on chain: {:?} for account: {}",
        pending_index,
        chain_enum,
        account_id
    );

    let chain =
        with_chain(&account_id, &chain_enum, |chain| chain.clone()).unwrap_or_else(panic_log);

    let result = chain.check_pending(pending_index).await;

    match result {
        Ok(_) => {
            with_chain_mut(&account_id, chain_enum, |chain| {
                chain.remove_pending(pending_index)
            })
            .unwrap_or_else(panic_log);
        }
        Err(err) => panic_log(err),
    }
}

#[update(guard = "caller_is_owner")]
async fn account_add_pending(account_id: AccountId, chain: ChainEnum, pending: PendingEnum) {
    log_cycle!(
        "Add pending: {:?} on chain: {:?} for account: {}",
        pending,
        chain,
        account_id
    );

    with_chain_mut(&account_id, chain, |chain| chain.add_pending(pending))
        .unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
async fn account_remove_pending(account_id: AccountId, chain: ChainEnum, pending_index: usize) {
    log_cycle!(
        "Remove pending: {} on chain: {:?} for account: {}",
        pending_index,
        chain,
        account_id
    );

    with_chain_mut(&account_id, chain, |chain| {
        chain.remove_pending(pending_index)
    })
    .unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
async fn account_swap_btc_to_ckbtc(
    account_id: AccountId,
    network: BitcoinNetwork,
    amount: Satoshi,
) -> BtcPending {
    log_cycle!(
        "Swap {} BTC to CKBTC on network: {} for account: {}",
        amount,
        network,
        account_id
    );

    let btc = with_chain(&account_id, &ChainEnum::BTC(network), |chain| chain.btc())
        .unwrap_or_else(panic_log)
        .unwrap_or_else(panic_log);

    let result = btc.swap_to_ckbtc(amount).await;

    match result {
        Ok(pending) => {
            with_chain_mut(&account_id, ChainEnum::BTC(network), |chain| {
                chain.add_pending(pending.clone().into())
            })
            .unwrap_or_else(panic_log);

            pending
        }
        Err(err) => panic_log(err),
    }
}

#[update(guard = "caller_is_owner")]
async fn account_swap_ckbtc_to_btc(
    account_id: AccountId,
    network: BitcoinNetwork,
    retrieve_address: String,
    amount: Satoshi,
) -> TransferBlockIndex {
    log_cycle!(
        "Swap {} CKBTC to BTC on network: {} for account: {}",
        amount,
        network,
        account_id
    );

    let ckbtc = with_chain(&account_id, &ChainEnum::CKBTC(network), |chain| {
        chain.ckbtc()
    })
    .unwrap_or_else(panic_log)
    .unwrap_or_else(panic_log);

    let result = ckbtc.swap_to_btc(retrieve_address, amount).await;

    match result {
        Ok(result) => {
            let block_index = result.block_index;

            with_chain_mut(&account_id, ChainEnum::CKBTC(network), |chain| {
                let pending = PendingEnum::new_ckbtc(block_index, None);

                chain.add_pending(pending)
            })
            .unwrap_or_else(panic_log);

            block_index
        }
        Err(err) => panic_log(err),
    }
}

#[update(guard = "caller_is_owner")]
async fn account_top_up_and_notify(
    account_id: AccountId,
    amount: ICPToken,
    canister_id: Option<CanisterId>,
) -> Result<u128, String> {
    log_cycle!(
        "Top up {} ICP for account: {} to canister: {:?}",
        amount,
        account_id,
        canister_id
    );

    let icp = with_chain(&account_id, &ChainEnum::ICP, |chain| chain.icp())
        .unwrap_or_else(panic_log)
        .unwrap_or_else(panic_log);

    let canister_id = canister_id.unwrap_or(ic_cdk::id());

    let block_index = icp
        .top_up(canister_id, amount)
        .await
        .unwrap_or_else(panic_log);

    let notify_result = icp.notify_top_up(canister_id, block_index).await.unwrap();

    match notify_result {
        NotifyTopUpResult::Ok(cycles) => Ok(cycles),
        NotifyTopUpResult::Err(err) => {
            with_chain_mut(&account_id, ChainEnum::ICP, |chain| {
                let pending = PendingEnum::new_icp(block_index, canister_id.to_string());

                chain.add_pending(pending)
            })
            .unwrap();

            Err(err.to_string())
        }
    }
}

#[update(guard = "caller_is_owner")]
async fn account_create_address(account_id: AccountId, chain_enum: ChainEnum) {
    log_cycle!(
        "Create address for account: {} on chain: {:?}",
        account_id,
        chain_enum
    );

    let mut ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(panic_log);

    let ecdsa = match chain_enum {
        ChainEnum::BTC(_) | ChainEnum::EVM(_) => {
            if !ledger.is_public_key_set() {
                let ecdsa = ledger
                    .subaccount
                    .ecdsa_public_key()
                    .await
                    .unwrap_or_else(panic_log);

                ledger
                    .set_ecdsa_public_key(ecdsa.clone())
                    .unwrap_or_else(panic_log);

                Some(ecdsa)
            } else {
                None
            }
        }
        _ => None,
    };

    let chain = ledger
        .new_chain(chain_enum.clone())
        .await
        .unwrap_or_else(panic_log);

    // Check if the chain is ckbtc and update balance for any pending balance
    if chain_enum.is_ckbtc() {
        // We don't care about the result. If it fails, it will be retried later
        let _ = chain.ckbtc().unwrap().update_balance();
    }

    with_ledger_mut(&account_id, |ledger| {
        if let Some(ecdsa) = ecdsa {
            ledger.set_ecdsa_public_key(ecdsa).unwrap_or_else(panic_log);
        }

        ledger.insert_chain(chain_enum, chain)
    })
    .unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
async fn account_btc_fees(network: BitcoinNetwork, num_blocks: u8) -> u64 {
    log_cycle!(
        "Get fees for network: {} with {} blocks",
        network,
        num_blocks
    );

    let rate = network.fee_rate(num_blocks).await;

    match rate {
        Ok(rate) => rate,
        Err(err) => panic_log(err),
    }
}

#[update(guard = "caller_is_owner")]
fn reset_accounts() {
    log_cycle!("Reset accounts");

    with_wallet_mut(|s| s.reset_accounts());
}

#[query(guard = "caller_is_owner")]
fn setting_and_signer() -> WalletSettings {
    with_setting(|s| s.clone())
}

#[update(guard = "caller_is_owner")]
async fn add_controller_and_update(controller_id: ControllerId, name: String) {
    log_cycle!("Add controller: {} with name: {}", controller_id, name);

    let mut settings = with_setting(|s| s.clone());

    settings
        .add_controller_and_update(controller_id, name)
        .await
        .unwrap_or_else(panic_log);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[update(guard = "caller_is_owner")]
async fn update_controller(controller_map: AppControllerMap) -> AppControllerMap {
    log_cycle!("Update controller: {:?}", controller_map);

    let mut settings = with_setting(|s| s.clone());

    settings
        .update_controller_and_update(controller_map)
        .await
        .unwrap_or_else(panic_log);

    with_wallet_mut(|w| w.set_setting(settings));

    with_setting(|s| s.controllers().clone())
}

#[update(guard = "caller_is_owner")]
async fn update_settings() {
    log_cycle!("Update settings");

    let mut settings = with_setting(|s| s.clone());

    settings.update_settings().await.unwrap_or_else(panic_log);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[update(guard = "caller_is_owner")]
async fn refresh_settings() {
    log_cycle!("Refresh settings");

    let mut settings = with_setting(|s| s.clone());

    settings.refresh_settings().await.unwrap_or_else(panic_log);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[update(guard = "caller_is_owner")]
fn add_setting_metadata(key: String, value: Value) {
    log_cycle!("Add metadata: {} with value: {}", key, value);

    with_setting_mut(|s| s.add_metadata(key, value));
}

#[update(guard = "caller_is_owner")]
fn remove_setting_metadata(key: String) {
    log_cycle!("Remove metadata: {}", key);

    with_setting_mut(|s| s.remove_metadata(&key));
}

#[update(guard = "caller_is_owner")]
async fn report_bug(system_canister_id: CanisterId, message: String) {
    log_cycle!("Report bug: {}", message);

    let request_args = AppBug {
        canister_id: ic_cdk::id(),
        description: message,
        version: version(),
        logs: export_log_messages_page(0, Some(10)),
        name: name(),
    };

    let _: () = ic_cdk::call(system_canister_id, "report_bug", (request_args,))
        .await
        .map_err(|err| format!("Error calling system canister: {:?}", err))
        .unwrap_or_else(panic_log);
}

#[update(guard = "caller_is_owner")]
async fn init_wallet(controller_map: AppControllerMap, metadata: Option<Metadata>) {
    log_cycle!("Initialize wallet: {:?}", controller_map);

    if with_wallet(|w| w.is_initialised()) {
        return panic_log(WalletError::WalletAlreadyInitialized);
    }

    let mut setting = WalletSettings::new(controller_map, metadata);

    setting.update_settings().await.unwrap_or_else(panic_log);

    with_wallet_mut(|w| w.init_wallet(setting));
}

#[update(guard = "caller_is_owner")]
async fn upgrage_wallet() {
    log_cycle!("Upgrade wallet");

    let canister_id = ic_cdk::id();
    let wasm_module = with_wasm_cache(|w| {
        if w.is_empty() {
            return panic_log(WalletError::WasmNotLoaded);
        }
        w.bytes()
    });

    let args = InstallCodeArgument {
        canister_id,
        wasm_module,
        arg: Vec::new(),
        mode: CanisterInstallMode::Upgrade,
    };

    install_code(args).await.unwrap();
}

#[update(guard = "caller_is_owner")]
async fn uninstall_wallet() {
    log_cycle!("Uninstall wallet");

    let canister_id = ic_cdk::id();

    let args = CanisterIdRecord { canister_id };

    uninstall_code(args).await.unwrap();
}

#[update(guard = "caller_is_owner")]
async fn status() -> AppStatus {
    log_cycle!("Get status");

    let canister_id = ic_cdk::api::id();

    let version = version();
    let name = name();

    let canister_status = Management::canister_status(canister_id)
        .await
        .unwrap_or_else(panic_log);

    let account_status = with_wallet(|s| s.account_status());
    let status_at = NanoTimeStamp::now();

    AppStatus {
        canister_id,
        name,
        version,
        status_at,
        canister_status,
        account_status,
    }
}

#[query]
fn canister_cycle_balance() -> u128 {
    ic_cdk::api::canister_balance128()
}

#[query]
fn canister_version() -> u64 {
    ic_cdk::api::canister_version()
}

#[query]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[query]
fn name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query(guard = "caller_is_owner")]
fn wasm_details() -> WasmDetails {
    with_wasm_cache(|w| {
        let hash = w.hash();
        let size = w.len();

        WasmDetails { hash, size }
    })
}

#[query(guard = "caller_is_owner")]
fn wasm_hash_string() -> String {
    with_wasm_cache(|w| w.hash_string())
}

#[query(guard = "caller_is_owner")]
fn wasm_hash() -> WasmHash {
    with_wasm_cache(|w| w.hash())
}

#[update(guard = "caller_is_owner")]
fn load_wasm(blob: Vec<u8>) -> WasmSize {
    log_cycle!("Load wasm");

    with_wasm_mut_cache(|w| w.load(&blob))
}

#[update(guard = "caller_is_owner")]
fn unload_wasm() -> WasmSize {
    log_cycle!("Unload wasm");

    with_wasm_mut_cache(|w| w.unload())
}

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

ic_cdk::export_candid!();
