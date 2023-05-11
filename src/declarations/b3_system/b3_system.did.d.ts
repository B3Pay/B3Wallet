import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
  'deprecated' : boolean,
}
export interface ReleaseArgs {
  'features' : [] | [Array<string>],
  'size' : bigint,
  'version' : string,
}
export type Result = { 'Ok' : Signer } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Release } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : LoadRelease } |
  { 'Err' : string };
export interface Signer {
  'updated_at' : bigint,
  'owner' : Principal,
  'signer_id' : [] | [Principal],
  'created_at' : bigint,
}
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'create_signer' : ActorMethod<[], Result>,
  'deprecate_release' : ActorMethod<[string], Result_1>,
  'get_release' : ActorMethod<[string], Result_2>,
  'get_release_by_index' : ActorMethod<[bigint], Result_2>,
  'get_signer' : ActorMethod<[], [] | [Signer]>,
  'get_signer_id' : ActorMethod<[Principal], [] | [Principal]>,
  'get_signers' : ActorMethod<[], Array<Principal>>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'latest_release' : ActorMethod<[], Release>,
  'load_release' : ActorMethod<[Uint8Array | number[], ReleaseArgs], Result_3>,
  'releases' : ActorMethod<[], Array<Release>>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_latest_release' : ActorMethod<[], undefined>,
  'remove_release' : ActorMethod<[string], Result_1>,
  'remove_signer' : ActorMethod<[Principal], undefined>,
  'update_release' : ActorMethod<[ReleaseArgs], Result_1>,
}
