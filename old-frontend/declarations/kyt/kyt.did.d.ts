import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Alert {
  'service' : [] | [string],
  'level' : { 'Low' : null } |
    { 'High' : null } |
    { 'Medium' : null } |
    { 'Severe' : null },
  'category' : [] | [string],
  'exposure_type' : { 'Indirect' : null } |
    { 'Direct' : null },
}
export interface DepositRequest {
  'txid' : Uint8Array | number[],
  'vout' : number,
  'caller' : Principal,
}
export type FetchUtxoAlertsError = { 'TemporarilyUnavailable' : string };
export type FetchWithdrawalAlertsError = { 'TemporarilyUnavailable' : string };
export interface InitArg {
  'maintainers' : Array<Principal>,
  'mode' : Mode,
  'minter_id' : Principal,
}
export type LifecycleArg = { 'UpgradeArg' : UpgradeArg } |
  { 'InitArg' : InitArg };
export type Mode = { 'RejectAll' : null } |
  { 'Normal' : null } |
  { 'AcceptAll' : null };
export interface Response {
  'provider' : Principal,
  'alerts' : Array<Alert>,
  'external_id' : string,
}
export interface SetApiKeyArg { 'api_key' : string }
export interface UpgradeArg {
  'maintainers' : [] | [Array<Principal>],
  'mode' : [] | [Mode],
  'minter_id' : [] | [Uint8Array | number[]],
}
export interface WithdrawalAttempt {
  'id' : string,
  'timestamp_nanos' : bigint,
  'address' : string,
  'caller' : Principal,
  'amount' : bigint,
}
export interface _SERVICE {
  'fetch_utxo_alerts' : ActorMethod<
    [DepositRequest],
    { 'Ok' : Response } |
      { 'Err' : FetchUtxoAlertsError }
  >,
  'fetch_withdrawal_alerts' : ActorMethod<
    [WithdrawalAttempt],
    { 'Ok' : Response } |
      { 'Err' : FetchWithdrawalAlertsError }
  >,
  'set_api_key' : ActorMethod<[SetApiKeyArg], undefined>,
  'txid_to_bytes' : ActorMethod<[string], Uint8Array | number[]>,
}
