import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Bug {
  'logs' : Array<string>,
  'name' : string,
  'canister_id' : Principal,
  'description' : string,
  'version' : string,
}
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
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
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
export interface _SERVICE {
  'add_wallet_canister' : ActorMethod<[Principal], undefined>,
  'change_wallet_canister' : ActorMethod<[Principal, bigint], undefined>,
  'clear_bugs' : ActorMethod<[Principal], undefined>,
  'create_wallet_canister' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], undefined>,
  'get_bugs' : ActorMethod<[Principal], Array<Bug>>,
  'get_canister_version' : ActorMethod<[Principal], string>,
  'get_canister_version_by_user' : ActorMethod<
    [Uint8Array | number[], bigint],
    string
  >,
  'get_canisters' : ActorMethod<[], Array<Principal>>,
  'get_create_canister_wallet_cycle' : ActorMethod<[], bigint>,
  'get_release' : ActorMethod<[string], Release>,
  'get_release_by_hash_string' : ActorMethod<[Uint8Array | number[]], Release>,
  'get_states' : ActorMethod<[], UserState>,
  'get_user_ids' : ActorMethod<[], Array<Uint8Array | number[]>>,
  'get_user_states' : ActorMethod<[], Array<UserState>>,
  'install_wallet_canister' : ActorMethod<[Principal], Result>,
  'latest_release' : ActorMethod<[], Release>,
  'load_release' : ActorMethod<
    [Uint8Array | number[], ReleaseArgs],
    LoadRelease
  >,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Release>,
  'remove_wallet_canister' : ActorMethod<[Uint8Array | number[]], undefined>,
  'report_bug' : ActorMethod<[Bug], undefined>,
  'status' : ActorMethod<[], SystemCanisterStatus>,
  'update_release' : ActorMethod<[ReleaseArgs], undefined>,
  'user_canister_status' : ActorMethod<[Principal], UserCanisterStatus>,
  'version' : ActorMethod<[], string>,
}
