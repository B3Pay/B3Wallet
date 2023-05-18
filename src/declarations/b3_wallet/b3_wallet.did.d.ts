import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsCounter {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface AddSignerRequest {
  'name' : [] | [string],
  'role' : Roles,
  'signer_id' : Principal,
  'expires_at' : [] | [bigint],
}
export type BitcoinNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface BtcRequest {
  'deadline' : bigint,
  'address' : string,
  'amount' : bigint,
}
export interface CallRequest {
  'arg' : Uint8Array | number[],
  'canister_id' : Principal,
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
export interface ConfirmedRequest {
  'status' : RequestStatus,
  'request' : PendingRequest,
  'error' : string,
  'timestamp' : bigint,
}
export interface CreateAccountRequest {
  'env' : [] | [Environment],
  'name' : [] | [string],
}
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface EcdsaPublicKeyRequest { 'account_id' : string }
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export type EvmRequest = { 'EvmTranscationRequest' : EvmTranscationRequest } |
  { 'EvmSignMessageRequest' : EvmSignMessageRequest };
export interface EvmSignMessageRequest {
  'id' : bigint,
  'deadline' : bigint,
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
export interface EvmTranscationRequest {
  'account_id' : string,
  'transaction' : EvmTransaction,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface IcpRequest {
  'to' : Principal,
  'deadline' : bigint,
  'amount' : bigint,
}
export type InnerRequest = {
    'UpgradeCanisterRequest' : UpgradeCanisterRequest
  } |
  { 'RenameAccountRequest' : RenameAccountRequest } |
  { 'CreateAccountRequest' : CreateAccountRequest } |
  { 'RawRandRequest' : RawRandRequest } |
  { 'EcdsaPublicKeyRequest' : EcdsaPublicKeyRequest } |
  { 'QueryRequest' : QueryRequest } |
  { 'AddSignerRequest' : AddSignerRequest } |
  { 'CallRequest' : CallRequest } |
  { 'TopUpCanisterRequest' : TopUpCanisterRequest } |
  { 'UpdateCanisterSettingsRequest' : UpdateCanisterSettingsRequest };
export interface Ledger {
  'subaccount' : Uint8Array | number[],
  'public_keys' : PublicKeys,
}
export type Network = { 'BTC' : BitcoinNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'SNS' : string };
export interface PendingRequest {
  'id' : bigint,
  'signers' : Array<Principal>,
  'request' : Request,
  'role' : Roles,
  'deadline' : bigint,
}
export interface PublicKeys {
  'ecdsa' : [] | [Uint8Array | number[]],
  'addresses' : Array<[string, string]>,
  'identifier' : Uint8Array | number[],
}
export interface QueryRequest {
  'arg' : Uint8Array | number[],
  'canister_id' : Principal,
  'sender' : [] | [Principal],
  'method_name' : string,
}
export interface RawRandRequest { 'length' : number }
export interface RenameAccountRequest {
  'account_id' : string,
  'new_name' : string,
}
export type Request = { 'BtcRequest' : BtcRequest } |
  { 'EvmRequest' : EvmRequest } |
  { 'IcpRequest' : IcpRequest } |
  { 'InnerRequest' : InnerRequest };
export type RequestStatus = { 'Fail' : null } |
  { 'Success' : null } |
  { 'Pending' : null };
export type Roles = { 'User' : null } |
  { 'Canister' : null } |
  { 'Admin' : null };
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
  'account_status' : AccountsCounter,
}
export interface Tokens { 'e8s' : bigint }
export interface TopUpCanisterRequest {
  'fee' : [] | [Tokens],
  'account_id' : string,
  'canister_id' : [] | [Principal],
  'amount' : Tokens,
}
export interface UpdateCanisterSettingsRequest {
  'canister_id' : Principal,
  'settings' : CanisterSettings,
}
export interface UpgradeCanisterRequest {
  'wasm_hash_string' : string,
  'wasm_version' : string,
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
  'account_request_public_key' : ActorMethod<[string], Array<[string, string]>>,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'confirm_request' : ActorMethod<[bigint], ConfirmedRequest>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_accounts' : ActorMethod<[], Array<WalletAccount>>,
  'get_addresses' : ActorMethod<[string], Array<[string, string]>>,
  'get_confirmed' : ActorMethod<[bigint], ConfirmedRequest>,
  'get_confirmed_requests' : ActorMethod<[], Array<[bigint, ConfirmedRequest]>>,
  'get_requests' : ActorMethod<[], Array<PendingRequest>>,
  'get_signers' : ActorMethod<[], Array<[Principal, Signer]>>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'reinstall_wallet' : ActorMethod<[], undefined>,
  'request_account_rename' : ActorMethod<
    [RenameAccountRequest, [] | [bigint]],
    bigint
  >,
  'request_maker' : ActorMethod<[Request], bigint>,
  'request_sign_message' : ActorMethod<
    [string, Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'request_sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Uint8Array | number[]
  >,
  'request_update_settings' : ActorMethod<
    [UpdateCanisterSettingsRequest, [] | [bigint]],
    bigint
  >,
  'reset_wallet' : ActorMethod<[], undefined>,
  'signer_add' : ActorMethod<[Principal, Roles], Array<[Principal, Signer]>>,
  'signer_remove' : ActorMethod<[Principal], Array<[Principal, Signer]>>,
  'status' : ActorMethod<[], SignerCanisterStatus>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'upgrade_wallet' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
}
