import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'id' : string,
  'name' : string,
  'ledger' : Ledger,
  'canisters' : Array<[Principal, Allowance]>,
  'requests' : Array<[Principal, SignRequest]>,
  'signed' : SignedTransaction,
}
export interface Allowance {
  'updated_at' : bigint,
  'metadata' : Array<[string, string]>,
  'created_at' : bigint,
  'limit' : [] | [number],
  'expires_at' : [] | [bigint],
}
export type BitcoinNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
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
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface Ledger {
  'subaccount' : Uint8Array | number[],
  'public_keys' : PublicKeys,
}
export type Network = { 'BTC' : BitcoinNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'SNS' : string };
export type NotifyError = {
    'Refunded' : { 'block_index' : [] | [bigint], 'reason' : string }
  } |
  { 'InvalidTransaction' : string } |
  { 'Other' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'Processing' : null } |
  { 'TransactionTooOld' : bigint };
export type NotifyTopUpResult = { 'Ok' : bigint } |
  { 'Err' : NotifyError };
export interface PublicKeys {
  'ecdsa' : [] | [Uint8Array | number[]],
  'addresses' : Array<[string, string]>,
  'identifier' : Uint8Array | number[],
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : Array<[string, string]> } |
  { 'Err' : [RejectionCode, string] };
export type Result_1 = { 'Ok' : Tokens } |
  { 'Err' : [RejectionCode, string] };
export type Result_10 = { 'Ok' : Uint8Array | number[] } |
  { 'Err' : [RejectionCode, string] };
export type Result_11 = { 'Ok' : CanisterStatus } |
  { 'Err' : SignerError };
export type Result_12 = { 'Ok' : NotifyTopUpResult } |
  { 'Err' : [RejectionCode, string] };
export type Result_13 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Result_14 = { 'Ok' : Result_13 } |
  { 'Err' : [RejectionCode, string] };
export type Result_2 = { 'Ok' : Array<[Principal, Allowance]> } |
  { 'Err' : [RejectionCode, string] };
export type Result_3 = { 'Ok' : SignRequest } |
  { 'Err' : [RejectionCode, string] };
export type Result_4 = { 'Ok' : SignedTransaction } |
  { 'Err' : [RejectionCode, string] };
export type Result_5 = { 'Ok' : Principal } |
  { 'Err' : [RejectionCode, string] };
export type Result_6 = { 'Ok' : Account } |
  { 'Err' : [RejectionCode, string] };
export type Result_7 = { 'Ok' : string } |
  { 'Err' : [RejectionCode, string] };
export type Result_8 = { 'Ok' : bigint } |
  { 'Err' : [RejectionCode, string] };
export type Result_9 = { 'Ok' : null } |
  { 'Err' : [RejectionCode, string] };
export interface SetAllowance {
  'metadata' : Array<[string, string]>,
  'limit' : [] | [number],
  'expires_at' : [] | [bigint],
}
export interface SignRequest {
  'transaction' : Transaction,
  'deadline' : bigint,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface SignedTransaction {
  'data' : Uint8Array | number[],
  'timestamp' : bigint,
}
export type SignerError = { 'InvalidAddress' : null } |
  { 'MaximumDevelopmentAccountsReached' : null } |
  { 'PasswordIsInvalid' : null } |
  { 'CanisterError' : string } |
  { 'TransactionAlreadyRemoved' : null } |
  { 'CyclesMintingError' : string } |
  { 'MaximumAccountsReached' : null } |
  { 'AccountNotExists' : null } |
  { 'TransactionTypeNotFound' : null } |
  { 'RequestNotExists' : null } |
  { 'AccountLimitReached' : null } |
  { 'PublicKeyError' : string } |
  { 'TransactionNotPending' : null } |
  { 'PublicKeyAlreadyExists' : null } |
  { 'EnvironmentMismatch' : null } |
  { 'UnknownError' : null } |
  { 'InvalidEcdsaPublicKey' : null } |
  { 'InvalidSubaccount' : null } |
  { 'GenerateError' : string } |
  { 'InsufficientBalance' : null } |
  { 'MissingEcdsaPublicKey' : null } |
  { 'PasswordHashError' : null } |
  { 'CallerNotAuthorized' : null } |
  { 'ManagementCanisterError' : string } |
  { 'LedgerError' : string } |
  { 'InvalidPublicKey' : null } |
  { 'SignError' : string } |
  { 'PasswordNotSet' : null } |
  { 'AccountAlreadyExists' : null } |
  { 'CallerIsNotOwner' : null } |
  { 'ChainNotFound' : null } |
  { 'CallerIsNotWalletCanister' : null } |
  { 'TransactionNotFound' : null } |
  { 'ChainAlreadyExists' : null } |
  { 'CanisterStatusError' : string } |
  { 'MaximumProductionAccountsReached' : null };
export interface Tokens { 'e8s' : bigint }
export interface Transaction {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'transaction_type' : TransactionType,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'data' : string,
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : bigint,
  'access_list' : [] | [Array<[string, Array<string>]>],
  'gas_price' : [] | [bigint],
}
export type TransactionType = { 'EIP1559' : null } |
  { 'EIP2930' : null } |
  { 'Legacy' : null };
export type TransferError = {
    'TxTooOld' : { 'allowed_window_nanos' : bigint }
  } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'TxDuplicate' : { 'duplicate_of' : bigint } } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export interface _SERVICE {
  'account_addresses' : ActorMethod<[string], Result>,
  'account_balance' : ActorMethod<[string], Result_1>,
  'account_connected_canisters' : ActorMethod<[string], Result_2>,
  'account_requests' : ActorMethod<[string, Principal], Result_3>,
  'account_signed_transaction' : ActorMethod<[string], Result_4>,
  'change_owner' : ActorMethod<[Principal], Result_5>,
  'create_account' : ActorMethod<[[] | [Environment], [] | [string]], Result_6>,
  'generate_address' : ActorMethod<[string, Network], Result_7>,
  'get_account' : ActorMethod<[string], Result_6>,
  'get_accounts' : ActorMethod<[], Array<Account>>,
  'get_caller' : ActorMethod<[], Principal>,
  'get_owner' : ActorMethod<[], Principal>,
  'load_wasm' : ActorMethod<[Uint8Array | number[], string], Result_8>,
  'number_of_accounts' : ActorMethod<[], number>,
  'reintall_canister' : ActorMethod<[], undefined>,
  'request_allowance' : ActorMethod<
    [string, Principal, SetAllowance],
    Result_9
  >,
  'request_ecdsa_public_key' : ActorMethod<[string], Result_10>,
  'reset_accounts' : ActorMethod<[], undefined>,
  'reset_wasm' : ActorMethod<[], undefined>,
  'sign_message' : ActorMethod<[string, Uint8Array | number[]], Result_10>,
  'sign_request' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Result_3
  >,
  'sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Result_4
  >,
  'status' : ActorMethod<[], Result_11>,
  'topup_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    Result_12
  >,
  'transfer_icp' : ActorMethod<
    [string, Tokens, string, [] | [Tokens], [] | [bigint]],
    Result_14
  >,
  'update_canister_controllers' : ActorMethod<[Array<Principal>], Result_9>,
  'upgrade_canister' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_version' : ActorMethod<[], string>,
}
