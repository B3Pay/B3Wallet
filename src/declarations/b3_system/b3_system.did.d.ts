import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterStatus {
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_counter' : bigint,
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
export type Result_1 = { 'Ok' : null } |
  { 'Err' : SystemError };
export type Result_2 = { 'Ok' : Release } |
  { 'Err' : SystemError };
export type Result_3 = { 'Ok' : LoadRelease } |
  { 'Err' : SystemError };
export interface SignerCanister {
  'updated_at' : bigint,
  'canister_id' : [] | [Principal],
  'created_at' : bigint,
}
export type SystemError = { 'UserAlreadyExists' : null } |
  { 'WasmGetError' : string } |
  { 'CreateCanisterError' : string } |
  { 'SignerCanisterAlreadyExists' : string } |
  { 'EncodeError' : string } |
  { 'SignerCanisterRateError' : string } |
  { 'InstallArgError' : string } |
  { 'SignerCanisterNotFound' : null } |
  { 'OwnerMismatch' : { 'owner' : string, 'user' : string } } |
  { 'InvalidAccountIdentifier' : null } |
  { 'UpdateControllersError' : string } |
  { 'ReleaseNotFound' : null } |
  { 'WasmNotFound' : null } |
  { 'WasmInstallError' : string } |
  { 'SignerCanisterAlreadyInstalled' : null } |
  { 'SignerCanisterDoesNotExist' : string } |
  { 'InstallCodeError' : string } |
  { 'UserNotFound' : null } |
  { 'CanisterStatusError' : string } |
  { 'WasmAlreadyLoaded' : null } |
  { 'ReleaseAlreadyExists' : null };
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'change_signer_canister' : ActorMethod<[Principal], undefined>,
  'create_signer_canister' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], Result_1>,
  'get_canister' : ActorMethod<[], SignerCanister>,
  'get_canister_version' : ActorMethod<[Principal], string>,
  'get_canister_wasmhash' : ActorMethod<[Principal], Uint8Array | number[]>,
  'get_controllers' : ActorMethod<[], Array<Principal>>,
  'get_release' : ActorMethod<[string], Result_2>,
  'get_release_by_index' : ActorMethod<[bigint], Result_2>,
  'get_signer_canisters' : ActorMethod<[], Array<SignerCanister>>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'install_signer_canister' : ActorMethod<[[] | [Principal]], Result>,
  'latest_release' : ActorMethod<[], Release>,
  'load_release' : ActorMethod<[Uint8Array | number[], ReleaseArgs], Result_3>,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Result_2>,
  'remove_signer_canister' : ActorMethod<[Principal], undefined>,
  'reset_users' : ActorMethod<[], undefined>,
  'status' : ActorMethod<[], CanisterStatus>,
  'update_release' : ActorMethod<[ReleaseArgs], Result_1>,
  'version' : ActorMethod<[], string>,
}
