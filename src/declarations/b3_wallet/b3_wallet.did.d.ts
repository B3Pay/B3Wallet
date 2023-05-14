import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsCounters {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface AccountsStatus {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface AddSignerRequest {
  'id' : bigint,
  'name' : string,
  'role' : string,
  'canister_id' : Principal,
  'deadline' : bigint,
  'expires_at' : [] | [bigint],
}
export type BitcoinNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface CallRequest {
  'id' : bigint,
  'arg' : Uint8Array | number[],
  'canister_id' : Principal,
  'deadline' : bigint,
  'sender' : [] | [Principal],
  'cycles' : [] | [bigint],
  'method_name' : string,
}
export interface CanisterSettings {
  'freezing_threshold' : [] | [bigint],
  'controllers' : [] | [Array<Principal>],
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
}
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
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface EvmSignRequest {
  'id' : bigint,
  'transaction' : EvmTransaction,
  'deadline' : bigint,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface EvmTransaction {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'transaction_type' : EvmTransactionType,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'data' : string,
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : bigint,
  'access_list' : [] | [Array<[string, Array<string>]>],
  'gas_price' : [] | [bigint],
}
export type EvmTransactionType = { 'EIP1559' : null } |
  { 'EIP2930' : null } |
  { 'Legacy' : null };
export type InnerCanisterRequest = { 'AddSigner' : AddSignerRequest } |
  { 'Call' : CallRequest } |
  { 'TopUpCanister' : TopUpCanisterRequest } |
  { 'UpdateSettings' : UpdateSettingsRequest } |
  { 'UpdateCanister' : UpgradeCanisterRequest } |
  { 'RawRand' : RawRandRequest } |
  { 'Query' : QueryRequest };
export interface Ledger {
  'subaccount' : Uint8Array | number[],
  'public_keys' : PublicKeys,
}
export type Network = { 'BTC' : BitcoinNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'SNS' : string };
export interface PublicKeys {
  'ecdsa' : [] | [Uint8Array | number[]],
  'addresses' : Array<[string, string]>,
  'identifier' : Uint8Array | number[],
}
export interface QueryRequest {
  'id' : bigint,
  'arg' : Uint8Array | number[],
  'canister_id' : Principal,
  'deadline' : bigint,
  'sender' : [] | [Principal],
  'method_name' : string,
}
export interface RawRandRequest {
  'id' : bigint,
  'deadline' : bigint,
  'length' : number,
}
export type Roles = { 'Operator' : null } |
  { 'User' : null } |
  { 'Canister' : null } |
  { 'Admin' : null } |
  { 'Owner' : null };
export interface SendBitcoinRequest {
  'id' : bigint,
  'deadline' : bigint,
  'address' : string,
  'amount' : bigint,
}
export interface SendIcpRequest {
  'id' : bigint,
  'to' : Principal,
  'deadline' : bigint,
  'amount' : bigint,
}
export type SignRequest = { 'Evm' : EvmSignRequest } |
  { 'Icp' : SendIcpRequest } |
  { 'Bitcoin' : SendBitcoinRequest } |
  { 'InnerCanister' : InnerCanisterRequest };
export interface SignedTransaction {
  'data' : Uint8Array | number[],
  'timestamp' : bigint,
}
export interface Signer {
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'role' : Roles,
  'expires_at' : [] | [bigint],
}
export interface SignerCanisterStatus {
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : AccountsStatus,
}
export interface State {
  'confirms' : Array<[bigint, SignedTransaction]>,
  'accounts' : Array<[string, WalletAccount]>,
  'counters' : AccountsCounters,
  'requests' : Array<SignRequest>,
}
export interface Tokens { 'e8s' : bigint }
export interface TopUpCanisterRequest {
  'id' : bigint,
  'canister_id' : Principal,
  'deadline' : bigint,
  'amount' : bigint,
}
export interface UpdateSettingsArgument {
  'canister_id' : Principal,
  'settings' : CanisterSettings,
}
export interface UpdateSettingsRequest {
  'id' : bigint,
  'deadline' : bigint,
  'settings' : UpdateSettingsArgument,
}
export interface UpgradeCanisterRequest {
  'id' : bigint,
  'wasm_hash_string' : string,
  'wasm_version' : string,
  'deadline' : bigint,
  'wasm_hash' : Uint8Array | number[],
}
export interface WalletAccount {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'ledger' : Ledger,
}
export interface _SERVICE {
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    WalletAccount
  >,
  'account_generate_address' : ActorMethod<[string, Network], string>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_icp_balance' : ActorMethod<[string], Tokens>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_rename' : ActorMethod<[string, string], string>,
  'account_request_public_key' : ActorMethod<[string], Uint8Array | number[]>,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'add_signer' : ActorMethod<[Principal, Roles], Array<[Principal, Signer]>>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_accounts' : ActorMethod<[], Array<WalletAccount>>,
  'get_addresses' : ActorMethod<[string], Array<[string, string]>>,
  'get_sign_requests' : ActorMethod<[string, bigint], SignRequest>,
  'get_signed_transaction' : ActorMethod<[bigint], SignedTransaction>,
  'get_signers' : ActorMethod<[], Array<[Principal, Signer]>>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'reintall_canister' : ActorMethod<[], undefined>,
  'remove_signer' : ActorMethod<[Principal], Array<[Principal, Signer]>>,
  'request_sign_message' : ActorMethod<
    [string, Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'request_sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    SignedTransaction
  >,
  'reset_accounts' : ActorMethod<[], State>,
  'status' : ActorMethod<[], SignerCanisterStatus>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'update_canister_controllers' : ActorMethod<[Array<Principal>], undefined>,
  'upgrade_canister' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
}
