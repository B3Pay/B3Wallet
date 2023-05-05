import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : string,
  'keys' : Keys,
  'name' : string,
  'ecdsa' : Ecdsa,
  'canisters' : Array<[Principal, Allowance]>,
  'requests' : Array<[Principal, SignRequest]>,
  'signed' : SignedTransaction,
}
export interface Addresses { 'btc' : BtcAddresses, 'eth' : string }
export interface Allowance {
  'updated_at' : bigint,
  'metadata' : Array<[string, string]>,
  'created_at' : bigint,
  'limit' : [] | [number],
  'expires_at' : [] | [bigint],
}
export interface BtcAddresses { 'mainnet' : string, 'testnet' : string }
export interface CanisterStatus {
  'id' : Principal,
  'status' : CanisterStatusResponse,
  'status_at' : bigint,
  'version' : string,
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
export interface Ecdsa { 'env' : Environment, 'path' : Uint8Array | number[] }
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface Keys {
  'addresses' : Addresses,
  'bytes' : Uint8Array | number[],
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : Account } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : SignRequest } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<[Principal, Allowance]> } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Uint8Array | number[] } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : SignedTransaction } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : CanisterStatus } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : null } |
  { 'Err' : [RejectionCode, string] };
export interface SetAllowance {
  'metadata' : Array<[string, string]>,
  'limit' : [] | [number],
  'expires_at' : [] | [bigint],
}
export interface SignRequest {
  'transaction' : Transaction,
  'deadline' : bigint,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface SignedTransaction {
  'data' : Uint8Array | number[],
  'timestamp' : bigint,
}
export interface Transaction {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'transaction_type' : TransactionType,
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
export type TransactionType = { 'EIP1559' : null } |
  { 'EIP2930' : null } |
  { 'Legacy' : null };
export interface _SERVICE {
  'change_owner' : ActorMethod<[Principal], undefined>,
  'create_account' : ActorMethod<[[] | [Environment], [] | [string]], Result>,
  'get_account' : ActorMethod<[string], Account>,
  'get_account_requests' : ActorMethod<[string, Principal], Result_1>,
  'get_accounts' : ActorMethod<[], Array<Account>>,
  'get_addresses' : ActorMethod<[string], Addresses>,
  'get_caller' : ActorMethod<[], Principal>,
  'get_connected_canisters' : ActorMethod<[string], Result_2>,
  'get_owner' : ActorMethod<[], Principal>,
  'get_signed' : ActorMethod<[string], SignedTransaction>,
  'load_wasm' : ActorMethod<[Uint8Array | number[], string], Result_3>,
  'number_of_accounts' : ActorMethod<[], number>,
  'reintall_canister' : ActorMethod<[], undefined>,
  'request_allowance' : ActorMethod<
    [string, Principal, SetAllowance],
    Result_4
  >,
  'request_sign' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    SignRequest
  >,
  'reset_user' : ActorMethod<[], undefined>,
  'reset_wasm' : ActorMethod<[], undefined>,
  'sign_message' : ActorMethod<[string, Uint8Array | number[]], Result_5>,
  'sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Result_6
  >,
  'status' : ActorMethod<[], Result_7>,
  'update_canister_controllers' : ActorMethod<[Array<Principal>], Result_8>,
  'upgrade_canister' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_version' : ActorMethod<[], string>,
}
