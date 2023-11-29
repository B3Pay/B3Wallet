import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Bug {
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
  'hash' : Uint8Array | number[],
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
export type Result = { 'Ok' : UserState } |
  { 'Err' : string };
export interface SystemCanisterStatus {
  'user_status' : bigint,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export interface UserCanisterStatus {
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export interface UserState {
  'updated_at' : bigint,
  'created_at' : bigint,
  'canisters' : Array<Principal>,
}
export type UserStatus = { 'Unregistered' : null } |
  { 'SingleCanister' : Principal } |
  { 'MultipleCanister' : Array<Principal> } |
  { 'Registered' : null };
export interface _SERVICE {
  'add_wallet_canister' : ActorMethod<[Principal], undefined>,
  'clear_bugs' : ActorMethod<[Principal], undefined>,
  'create_wallet_canister' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], undefined>,
  'get_bugs' : ActorMethod<[Principal], Array<Bug>>,
  'get_canister_info' : ActorMethod<[Principal], CanisterInfoResponse>,
  'get_canister_version' : ActorMethod<[Principal], string>,
  'get_canisters' : ActorMethod<[], Array<Principal>>,
  'get_create_canister_wallet_cycle' : ActorMethod<[], bigint>,
  'get_release' : ActorMethod<[string], Release>,
  'get_release_by_hash_string' : ActorMethod<[Uint8Array | number[]], Release>,
  'get_states' : ActorMethod<[], UserState>,
  'get_user_canister_status' : ActorMethod<[Principal], UserCanisterStatus>,
  'get_user_ids' : ActorMethod<[], Array<Uint8Array | number[]>>,
  'get_user_states' : ActorMethod<[], Array<UserState>>,
  'get_user_status' : ActorMethod<[], UserStatus>,
  'install_wallet_canister' : ActorMethod<[Principal], Result>,
  'latest_release' : ActorMethod<[], Release>,
  'load_release' : ActorMethod<
    [Uint8Array | number[], ReleaseArgs],
    LoadRelease
  >,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Release>,
  'remove_user' : ActorMethod<[Principal], undefined>,
  'remove_wallet_canister' : ActorMethod<[Principal], undefined>,
  'report_bug' : ActorMethod<[Bug], undefined>,
  'status' : ActorMethod<[], SystemCanisterStatus>,
  'update_release' : ActorMethod<[ReleaseArgs], undefined>,
  'version' : ActorMethod<[], string>,
}
