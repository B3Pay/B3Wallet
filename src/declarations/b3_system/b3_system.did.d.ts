import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Controller {
  'updated_at' : bigint,
  'created_at' : bigint,
  'expires_at' : [] | [bigint],
}
export interface LoadRelease { 'total' : bigint, 'chunks' : bigint }
export interface _SERVICE {
  'add_controller' : ActorMethod<[Principal], undefined>,
  'get_controllers' : ActorMethod<[], Array<[Principal, Controller]>>,
  'get_releases_version' : ActorMethod<[], string>,
  'load_release' : ActorMethod<[Uint8Array | number[], string], LoadRelease>,
  'remove_controller' : ActorMethod<[Principal], undefined>,
  'reset_release' : ActorMethod<[], undefined>,
}
