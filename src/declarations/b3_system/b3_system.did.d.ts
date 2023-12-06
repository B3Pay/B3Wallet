import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AppBug {
  'logs' : Array<string>,
  'name' : string,
  'canister_id' : Principal,
  'description' : string,
  'version' : string,
}
export interface CanisterChange {
  'timestamp_nanos' : bigint,
  'canister_version' : bigint,
  'origin' : CanisterChangeOrigin,
  'details' : CanisterChangeDetails,
}
export type CanisterChangeDetails = { 'creation' : CreationRecord } |
  { 'code_deployment' : CodeDeploymentRecord } |
  { 'controllers_change' : CreationRecord } |
  { 'code_uninstall' : null };
export type CanisterChangeOrigin = { 'from_user' : FromUserRecord } |
  { 'from_canister' : FromCanisterRecord };
export interface CanisterInfoResponse {
  'controllers' : Array<Principal>,
  'module_hash' : [] | [Uint8Array | number[]],
  'recent_changes' : Array<CanisterChange>,
  'total_num_changes' : bigint,
}
export type CanisterInstallMode = { 'reinstall' : null } |
  { 'upgrade' : null } |
  { 'install' : null };
export interface CanisterStatusResponse {
  'status' : CanisterStatusType,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'query_stats' : QueryStats,
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
}
export type CanisterStatusType = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export interface CodeDeploymentRecord {
  'mode' : CanisterInstallMode,
  'module_hash' : Uint8Array | number[],
}
export interface CreationRecord { 'controllers' : Array<Principal> }
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface FromCanisterRecord {
  'canister_version' : [] | [bigint],
  'canister_id' : Principal,
}
export interface FromUserRecord { 'user_id' : Principal }
export interface LoadRelease {
  'total' : bigint,
  'version' : string,
  'chunks' : bigint,
}
export interface QueryStats {
  'response_payload_bytes_total' : bigint,
  'num_instructions_total' : bigint,
  'num_calls_total' : bigint,
  'request_payload_bytes_total' : bigint,
}
export interface Release {
  'features' : Array<string>,
  'date' : bigint,
  'name' : string,
  'size' : bigint,
  'version' : string,
  'deprecated' : boolean,
}
export interface ReleaseArgs {
  'features' : Array<string>,
  'name' : string,
  'size' : bigint,
  'version' : string,
}
export type Result = { 'Ok' : User } |
  { 'Err' : string };
export interface SystemCanisterStatus {
  'user_status' : bigint,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export interface User {
  'updated_at' : bigint,
  'metadata' : Array<[string, Value]>,
  'created_at' : bigint,
  'canisters' : Array<Principal>,
}
export interface UserCanisterStatus {
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export type UserStatus = { 'Unregistered' : null } |
  { 'SingleCanister' : Principal } |
  { 'MultipleCanister' : Array<Principal> } |
  { 'Registered' : null };
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value> };
export interface _SERVICE {
  'add_app' : ActorMethod<[Principal], undefined>,
  'clear_bugs' : ActorMethod<[Principal], undefined>,
  'create_app_canister' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], Release>,
  'get_app_version' : ActorMethod<[Principal], string>,
  'get_bugs' : ActorMethod<[Principal], Array<AppBug>>,
  'get_canister_info' : ActorMethod<[Principal], CanisterInfoResponse>,
  'get_canisters' : ActorMethod<[], Array<Principal>>,
  'get_create_canister_app_cycle' : ActorMethod<[], bigint>,
  'get_latest_release' : ActorMethod<[], Release>,
  'get_release' : ActorMethod<[string], Release>,
  'get_release_by_hash_string' : ActorMethod<[Uint8Array | number[]], Release>,
  'get_states' : ActorMethod<[], User>,
  'get_user_app_status' : ActorMethod<[Principal], UserCanisterStatus>,
  'get_user_ids' : ActorMethod<[], Array<Uint8Array | number[]>>,
  'get_user_states' : ActorMethod<[], Array<User>>,
  'get_user_status' : ActorMethod<[], UserStatus>,
  'install_app' : ActorMethod<[Principal], Result>,
  'load_release' : ActorMethod<
    [Uint8Array | number[], ReleaseArgs],
    LoadRelease
  >,
  'release_wasm_hash' : ActorMethod<[], Array<[string, Uint8Array | number[]]>>,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_app' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Release>,
  'remove_user' : ActorMethod<[Principal], undefined>,
  'report_bug' : ActorMethod<[AppBug], undefined>,
  'status' : ActorMethod<[], SystemCanisterStatus>,
  'update_release' : ActorMethod<[ReleaseArgs], undefined>,
  'version' : ActorMethod<[], string>,
}
