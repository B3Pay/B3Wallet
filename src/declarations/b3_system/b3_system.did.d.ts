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
  'size' : bigint,
  'version' : string,
  'deprecated' : boolean,
}
export interface ReleaseArgs {
  'features' : [] | [Array<string>],
  'size' : bigint,
  'version' : string,
}
export type Result = { 'Ok' : SignerCanister } |
  { 'Err' : string };
export interface SignerCanister {
  'updated_at' : bigint,
  'canister_id' : [] | [Principal],
  'created_at' : bigint,
}
export interface SystemCanisterStatus {
  'canister_id' : Principal,
  'user_status' : bigint,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
}
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'change_wallet_canister' : ActorMethod<[Principal], undefined>,
  'create_wallet_canister' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], undefined>,
  'get_canister' : ActorMethod<[], SignerCanister>,
  'get_canister_version' : ActorMethod<[Principal], string>,
  'get_canister_version_by_user' : ActorMethod<[Principal], string>,
  'get_controllers' : ActorMethod<[], Array<Principal>>,
  'get_release' : ActorMethod<[string], Release>,
  'get_release_by_index' : ActorMethod<[bigint], Release>,
  'get_signer_canisters' : ActorMethod<[], Array<SignerCanister>>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'get_wallet_release' : ActorMethod<[Principal], Release>,
  'install_wallet_canister' : ActorMethod<[[] | [Principal]], Result>,
  'latest_release' : ActorMethod<[], Release>,
  'load_release' : ActorMethod<
    [Uint8Array | number[], ReleaseArgs],
    LoadRelease
  >,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Release>,
  'remove_wallet_canister' : ActorMethod<[Principal], undefined>,
  'reset_users' : ActorMethod<[], undefined>,
  'status' : ActorMethod<[], SystemCanisterStatus>,
  'update_release' : ActorMethod<[ReleaseArgs], undefined>,
  'version' : ActorMethod<[], string>,
}
