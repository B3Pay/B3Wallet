import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'derivation' : Derivation,
  'public_key' : PublicKey,
  'name' : string,
  'chain_data' : Array<[bigint, ChainData]>,
}
export interface ChainData {
  'nonce' : bigint,
  'transactions' : Array<SignedTransaction>,
}
export interface Config {
  'env' : Environment,
  'sign_cycles' : bigint,
  'key_name' : string,
}
export interface Derivation {
  'path' : Uint8Array | number[],
  'config' : Config,
}
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface PublicKey {
  'address' : string,
  'bytes' : Uint8Array | number[],
}
export type Result = { 'Ok' : PublicKey } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : SignedTransaction } |
  { 'Err' : string };
export interface SignedTransaction {
  'status' : Status,
  'data' : Uint8Array | number[],
  'timestamp' : bigint,
}
export type Status = { 'Failed' : null } |
  { 'Success' : null } |
  { 'Pending' : null };
export interface _SERVICE {
  'create_account' : ActorMethod<[Environment, [] | [string]], Result>,
  'get_account' : ActorMethod<[number], Account>,
  'get_accounts' : ActorMethod<[], Array<Account>>,
  'get_public_key' : ActorMethod<[number], PublicKey>,
  'sign_transaction' : ActorMethod<
    [number, bigint, Uint8Array | number[]],
    Result_1
  >,
}
