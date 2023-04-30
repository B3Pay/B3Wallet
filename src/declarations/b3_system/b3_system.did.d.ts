import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Controller {
  'updated_at' : bigint,
  'created_at' : bigint,
  'expires_at' : [] | [bigint],
}
export interface LoadRelease { 'total' : bigint, 'chunks' : bigint }
export type Result = { 'Ok' : UserControl } |
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
  'get_controllers' : ActorMethod<[], Array<[Principal, Controller]>>,
  'get_releases_version' : ActorMethod<[], string>,
  'get_user_control' : ActorMethod<[], [] | [UserControl]>,
  'get_user_control_id' : ActorMethod<[Principal], [] | [Principal]>,
  'get_user_ids' : ActorMethod<[], Array<Principal>>,
  'load_release' : ActorMethod<[Uint8Array | number[], string], LoadRelease>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'remove_user_control' : ActorMethod<[Principal], undefined>,
  'reset_release' : ActorMethod<[], undefined>,
}
