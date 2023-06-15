import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsNonce {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface BtcChain { 'address' : string, 'btc_network' : Minter }
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
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
  'memo' : [] | [Uint8Array | number[]],
  'minter' : Minter,
  'ledger' : Principal,
  'account' : ICRCAccount,
  'created_at_time' : [] | [bigint],
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
export interface EvmChain { 'chain_id' : bigint, 'address' : string }
export interface GetUtxosResponse {
  'next_page' : [] | [Uint8Array | number[]],
  'tip_height' : number,
  'tip_block_hash' : Uint8Array | number[],
  'utxos' : Array<Utxo>,
}
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
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [Timestamp],
}
export interface IcrcChain {
  'fee' : [] | [bigint],
  'metadata' : Array<[string, ICRC1MetadataValue]>,
  'memo' : [] | [Uint8Array | number[]],
  'canister_id' : Principal,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [bigint],
}
export interface Ledger {
  'public_key' : [] | [Uint8Array | number[]],
  'pending_sends' : Array<[Minter, BigUint64Array | bigint[]]>,
  'subaccount' : Uint8Array | number[],
  'pending_receives' : Array<[Minter, string]>,
  'chains' : Array<[ChainEnum, Chain]>,
}
export type Minter = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface OutPoint { 'txid' : Uint8Array | number[], 'vout' : number }
export interface Outpoint { 'txid' : Uint8Array | number[], 'vout' : number }
export type RetrieveBtcStatus = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Pending' : null };
export interface Timestamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : Outpoint,
}
export type UtxoFilter = { 'page' : Uint8Array | number[] } |
  { 'min_confirmations' : number };
export type UtxoStatus = { 'ValueTooSmall' : Utxo_1 } |
  { 'Tainted' : Utxo_1 } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo_1,
    }
  } |
  { 'Checked' : Utxo_1 };
export interface Utxo_1 {
  'height' : number,
  'value' : bigint,
  'outpoint' : OutPoint,
}
export interface WalletAccount {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'ledger' : Ledger,
}
export interface WalletAccountView {
  'id' : string,
  'pending_send' : Array<[Minter, BigUint64Array | bigint[]]>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'pending_receive' : Array<[Minter, string]>,
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
export interface WasmDetails { 'hash' : Uint8Array | number[], 'size' : bigint }
export interface _SERVICE {
  'account_balance' : ActorMethod<[string, ChainEnum], bigint>,
  'account_balance_btc' : ActorMethod<
    [string, BtcNetwork, [] | [number]],
    bigint
  >,
  'account_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'account_btc_utxos' : ActorMethod<
    [string, BtcNetwork, [] | [UtxoFilter]],
    GetUtxosResponse
  >,
  'account_ckbtc_balance' : ActorMethod<[string, BtcNetwork], bigint>,
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_create_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_icp_balance' : ActorMethod<[string], bigint>,
  'account_icrc_balance' : ActorMethod<[string, Principal], bigint>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_remove_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_remove_pending_receive' : ActorMethod<
    [string, BtcNetwork],
    undefined
  >,
  'account_remove_pending_send' : ActorMethod<
    [string, BtcNetwork, bigint],
    undefined
  >,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send' : ActorMethod<[string, ChainEnum, string, bigint], undefined>,
  'account_send_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    string
  >,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_swap_btc_to_ckbtc' : ActorMethod<
    [string, BtcNetwork, bigint],
    string
  >,
  'account_swap_ckbtc_to_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'account_update_receive_pending' : ActorMethod<
    [string, BtcNetwork],
    Array<UtxoStatus>
  >,
  'canister_cycle_balance' : ActorMethod<[], bigint>,
  'canister_version' : ActorMethod<[], bigint>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], AccountsNonce>,
  'get_account_view' : ActorMethod<[string], WalletAccountView>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_addresses' : ActorMethod<[string], Array<[ChainEnum, string]>>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'name' : ActorMethod<[], string>,
  'reset_wallet' : ActorMethod<[], undefined>,
  'retrieve_btc_status' : ActorMethod<[Minter, bigint], RetrieveBtcStatus>,
  'status' : ActorMethod<[], WalletCanisterStatus>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'upgrage_wallet' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_details' : ActorMethod<[], WasmDetails>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
