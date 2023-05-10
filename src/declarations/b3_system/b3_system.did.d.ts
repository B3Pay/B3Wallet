import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
export interface LoadRelease {
  'total' : bigint,
  'version' : string,
  'chunks' : bigint,
}
export interface Release {
  'features' : [] | [Array<string>],
  'date' : bigint,
  'hash' : string,
  'size' : bigint,
  'version' : string,
}
export interface ReleaseArgs {
  'features' : [] | [Array<string>],
  'size' : bigint,
  'version' : string,
}
export type Result = { 'Ok' : UserControl } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : CanisterStatus } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : LoadRelease } |
  { 'Err' : string };
export interface UserControl {
  'updated_at' : bigint,
  'user_control_id' : [] | [Principal],
  'owner' : Principal,
  'created_at' : bigint,
}
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'create_user_control' : ActorMethod<[], Result>,
  'get_canister_status' : ActorMethod<[Principal], Result_1>,
  'get_controllers' : ActorMethod<[], Array<Principal>>,
  'get_latest_release' : ActorMethod<[], Release>,
  'get_release' : ActorMethod<[bigint], Release>,
  'get_releases' : ActorMethod<[], Array<Release>>,
  'get_user_control' : ActorMethod<[], [] | [UserControl]>,
  'get_user_control_id' : ActorMethod<[Principal], [] | [Principal]>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'load_release' : ActorMethod<[Uint8Array | number[], ReleaseArgs], Result_2>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_user_control' : ActorMethod<[Principal], undefined>,
}
