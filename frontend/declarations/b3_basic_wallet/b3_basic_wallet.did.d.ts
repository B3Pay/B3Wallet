import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsNonce {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface BtcChain {
  'pendings' : Array<BtcPending>,
  'subaccount' : Uint8Array | number[],
  'ecdsa_public_key' : Uint8Array | number[],
  'address' : string,
  'btc_network' : Minter,
  'min_confirmations' : [] | [number],
}
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface BtcPending { 'txid' : string, 'account' : string }
export interface CanisterStatusResponse {
  'status' : CanisterStatusType,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
}
export type CanisterStatusType = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export type Chain = { 'EvmChain' : EvmChain } |
  { 'BtcChain' : BtcChain } |
  { 'IcpChain' : IcpChain } |
  { 'IcrcChain' : IcrcChain } |
  { 'CkbtcChain' : CkbtcChain };
export type ChainEnum = { 'BTC' : Minter } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'ICRC' : Principal } |
  { 'CKBTC' : Minter };
export interface CkbtcChain {
  'fee' : [] | [bigint],
  'pendings' : Array<CkbtcPending>,
  'memo' : [] | [Uint8Array | number[]],
  'minter' : Minter,
  'ledger' : Principal,
  'account' : ICRCAccount,
  'created_at_time' : [] | [bigint],
}
export interface CkbtcPending { 'block_index' : bigint, 'txid' : [] | [bigint] }
export interface Controller {
  'metadata' : Array<[string, string]>,
  'name' : string,
}
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface EvmChain {
  'pendings' : Array<EvmPending>,
  'chain_id' : bigint,
  'address' : string,
}
export interface EvmPending { 'block_index' : bigint }
export type ICRC1MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface ICRCAccount {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface IcpChain {
  'fee' : Tokens,
  'pendings' : Array<IcpPending>,
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [Timestamp],
}
export interface IcpPending { 'block_index' : bigint, 'canister_id' : string }
export interface IcrcChain {
  'fee' : [] | [bigint],
  'metadata' : Array<[string, ICRC1MetadataValue]>,
  'pendings' : Array<IcrcPending>,
  'memo' : [] | [Uint8Array | number[]],
  'canister_id' : Principal,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [bigint],
}
export interface IcrcPending { 'tx_index' : bigint, 'block_index' : bigint }
export interface InititializeWalletArgs {
  'controllers' : Array<[Principal, Controller]>,
  'metadata' : [] | [Array<[string, string]>],
}
export interface Ledger {
  'public_key' : [] | [Uint8Array | number[]],
  'subaccount' : Uint8Array | number[],
  'chains' : Array<[ChainEnum, Chain]>,
}
export type Minter = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface OutPoint { 'txid' : Uint8Array | number[], 'vout' : number }
export type PendingEnum = { 'BTC' : BtcPending } |
  { 'EVM' : EvmPending } |
  { 'ICP' : IcpPending } |
  { 'ICRC' : IcrcPending } |
  { 'CKBTC' : CkbtcPending };
export type Result = { 'Ok' : bigint } |
  { 'Err' : string };
export type RetrieveBtcStatus = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Pending' : null };
export type SendResult = { 'BTC' : string } |
  { 'EVM' : null } |
  { 'ICP' : bigint } |
  { 'ICRC' : bigint } |
  { 'CKBTC' : bigint };
export interface Timestamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : OutPoint,
}
export type UtxoStatus = { 'ValueTooSmall' : Utxo } |
  { 'Tainted' : Utxo } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo,
    }
  } |
  { 'Checked' : Utxo };
export interface WalletAccount {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'ledger' : Ledger,
}
export interface WalletAccountView {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'pendings' : Array<PendingEnum>,
  'name' : string,
  'hidden' : boolean,
  'addresses' : Array<[ChainEnum, string]>,
  'environment' : Environment,
}
export interface WalletCanisterStatus {
  'name' : string,
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : AccountsNonce,
}
export interface WalletSettings {
  'freezing_threshold' : [] | [bigint],
  'controllers' : Array<[Principal, Controller]>,
  'initialised' : boolean,
  'metadata' : Array<[string, string]>,
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
}
export interface WasmDetails { 'hash' : Uint8Array | number[], 'size' : bigint }
export interface _SERVICE {
  'account_add_pending' : ActorMethod<
    [string, ChainEnum, PendingEnum],
    undefined
  >,
  'account_balance' : ActorMethod<[string, ChainEnum], bigint>,
  'account_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'account_check_pending' : ActorMethod<[string, ChainEnum, bigint], undefined>,
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_create_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_remove_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_remove_pending' : ActorMethod<
    [string, ChainEnum, bigint],
    undefined
  >,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send' : ActorMethod<[string, ChainEnum, string, bigint], SendResult>,
  'account_swap_btc_to_ckbtc' : ActorMethod<
    [string, BtcNetwork, bigint],
    BtcPending
  >,
  'account_swap_ckbtc_to_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal]],
    Result
  >,
  'account_update_balance' : ActorMethod<
    [string, BtcNetwork],
    Array<UtxoStatus>
  >,
  'add_controller_and_update' : ActorMethod<
    [Principal, string, [] | [Array<[string, string]>]],
    undefined
  >,
  'add_setting' : ActorMethod<[string, string], undefined>,
  'canister_cycle_balance' : ActorMethod<[], bigint>,
  'canister_version' : ActorMethod<[], bigint>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], AccountsNonce>,
  'get_account_view' : ActorMethod<[string], WalletAccountView>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_addresses' : ActorMethod<[string], Array<[ChainEnum, string]>>,
  'init_wallet' : ActorMethod<[InititializeWalletArgs], undefined>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'name' : ActorMethod<[], string>,
  'refresh_settings' : ActorMethod<[], undefined>,
  'remove_setting' : ActorMethod<[string], undefined>,
  'reset_accounts' : ActorMethod<[], undefined>,
  'retrieve_btc_status' : ActorMethod<[Minter, bigint], RetrieveBtcStatus>,
  'setting' : ActorMethod<[], WalletSettings>,
  'status' : ActorMethod<[], WalletCanisterStatus>,
  'uninstall_wallet' : ActorMethod<[], undefined>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'update_controller' : ActorMethod<
    [Array<[Principal, Controller]>],
    Array<[Principal, Controller]>
  >,
  'update_settings' : ActorMethod<[], undefined>,
  'upgrage_wallet' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_details' : ActorMethod<[], WasmDetails>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
