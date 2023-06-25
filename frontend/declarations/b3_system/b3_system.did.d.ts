import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
export interface LoadRelease {
  'total' : bigint,
  'version' : string,
  'chunks' : bigint,
}
export interface Release {
  'features' : [] | [Array<string>],
  'date' : bigint,
  'hash' : Uint8Array | number[],
  'name' : string,
  'size' : bigint,
  'version' : string,
  'deprecated' : boolean,
}
export interface ReleaseArgs {
  'features' : [] | [Array<string>],
  'name' : string,
  'size' : bigint,
  'version' : string,
}
export type ReleaseName = { 'b3_wallet' : null } |
  { 'Custom' : string } |
  { 'b3_multi_sig_wallet' : null } |
  { 'b3_basic_wallet' : null };
export type Result = { 'Ok' : UserState } |
  { 'Err' : string };
export interface SystemCanisterStatus {
  'canister_id' : Principal,
  'user_status' : bigint,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export interface UserState {
  'updated_at' : bigint,
  'created_at' : bigint,
  'canisters' : Array<Principal>,
}
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'add_wallet_canister' : ActorMethod<[Principal], undefined>,
  'change_wallet_canister' : ActorMethod<[Principal, bigint], undefined>,
  'create_wallet_canister' : ActorMethod<[string], Result>,
  'deprecate_release' : ActorMethod<[string, string], undefined>,
  'get_canister_version' : ActorMethod<[Principal], string>,
  'get_canister_version_by_user' : ActorMethod<[Principal, bigint], string>,
  'get_canisters' : ActorMethod<[], Array<Principal>>,
  'get_controllers' : ActorMethod<[], Array<Principal>>,
  'get_release' : ActorMethod<[string, string], Release>,
  'get_release_by_hash_string' : ActorMethod<
    [string, Uint8Array | number[]],
    Release
  >,
  'get_release_by_index' : ActorMethod<[string, bigint], Release>,
  'get_states' : ActorMethod<[], UserState>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'get_user_states' : ActorMethod<[], Array<UserState>>,
  'install_wallet_canister' : ActorMethod<[string, Principal], Result>,
  'latest_release' : ActorMethod<[string], Release>,
  'load_release' : ActorMethod<
    [string, Uint8Array | number[], ReleaseArgs],
    LoadRelease
  >,
  'release_map' : ActorMethod<[], Array<[ReleaseName, Array<Release>]>>,
  'releases' : ActorMethod<[string], Array<Release>>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[string], undefined>,
  'remove_release' : ActorMethod<[string, string], Release>,
  'remove_wallet_canister' : ActorMethod<[Principal], undefined>,
  'reset_users' : ActorMethod<[], undefined>,
  'status' : ActorMethod<[], SystemCanisterStatus>,
  'update_release' : ActorMethod<[string, ReleaseArgs], undefined>,
  'version' : ActorMethod<[], string>,
}
