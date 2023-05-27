import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsCounter {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface AddSignerRequest {
  'threshold' : [] | [number],
  'name' : [] | [string],
  'role' : Roles,
  'signer_id' : Principal,
  'expires_at' : [] | [bigint],
}
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export type BtcRequest = { 'BtcTransferRequest' : BtcTransferRequest };
export interface BtcTransferRequest {
  'deadline' : bigint,
  'address' : string,
  'amount' : bigint,
}
export interface CanisterSettings {
  'freezing_threshold' : [] | [bigint],
  'controllers' : [] | [Array<Principal>],
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
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
export type Chains = { 'BTC' : BtcNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'SNS' : string };
export interface ConfirmedRequest {
  'status' : RequestStatus,
  'request' : PendingRequest,
  'error' : string,
  'message' : ConsentMessageResponse,
  'timestamp' : bigint,
}
export interface ConsendInfo { 'consent_message' : string, 'language' : string }
export type ConsentMessageResponse = { 'MalformedCall' : ErrorInfo } |
  { 'Valid' : ConsendInfo } |
  { 'Other' : string } |
  { 'Forbidden' : ErrorInfo };
export interface CreateAccountRequest {
  'env' : [] | [Environment],
  'name' : [] | [string],
}
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
export interface ErrorInfo { 'description' : string, 'error_code' : bigint }
export interface EvmDeployContractRequest {
  'account_id' : string,
  'hex_byte_code' : Uint8Array | number[],
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
}
export type EvmRequest = {
    'EvmDeployContractRequest' : EvmDeployContractRequest
  } |
  { 'EvmSignRawTransactionRequest' : EvmSignRawTransactionRequest } |
  { 'EvmSignMessageRequest' : EvmSignMessageRequest } |
  { 'EvmTransferErc20Request' : EvmTransferErc20Request } |
  { 'EvmSignTranscationRequest' : EvmSignTranscationRequest } |
  { 'EvmTransferEthRequest' : EvmTransferEthRequest };
export interface EvmSignMessageRequest {
  'account_id' : string,
  'message' : Uint8Array | number[],
}
export interface EvmSignRawTransactionRequest {
  'account_id' : string,
  'hex_raw_tx' : Uint8Array | number[],
  'chain_id' : bigint,
}
export interface EvmSignTranscationRequest {
  'account_id' : string,
  'transaction' : EvmTransaction,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface EvmTransaction {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'transaction_type' : EvmTransactionType,
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
export type EvmTransactionType = { 'EIP1559' : null } |
  { 'EIP2930' : null } |
  { 'Legacy' : null };
export interface EvmTransferErc20Request {
  'account_id' : string,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'address' : string,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
  'contract_address' : string,
}
export interface EvmTransferEthRequest {
  'to' : string,
  'account_id' : string,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
}
export interface GetUtxosResponse {
  'next_page' : [] | [Uint8Array | number[]],
  'tip_height' : number,
  'tip_block_hash' : Uint8Array | number[],
  'utxos' : Array<Utxo>,
}
export interface HideAccountRequest { 'account_id' : string }
export type IcpRequest = { 'IcpTransferRequest' : IcpTransferRequest } |
  { 'TopUpCanisterRequest' : TopUpCanisterRequest };
export interface IcpTransferRequest {
  'to' : Uint8Array | number[],
  'fee' : [] | [Tokens],
  'account_id' : string,
  'memo' : [] | [bigint],
  'amount' : Tokens,
}
export type InnerRequest = {
    'UpgradeCanisterRequest' : UpgradeCanisterRequest
  } |
  { 'RenameAccountRequest' : RenameAccountRequest } |
  { 'UnhideAccountRequest' : HideAccountRequest } |
  { 'CreateAccountRequest' : CreateAccountRequest } |
  { 'RemoveAccountRequest' : HideAccountRequest } |
  { 'RemoveSignerRequest' : RemoveSignerRequest } |
  { 'UpdateSignerThresholdRequest' : UpdateSignerThresholdRequest } |
  { 'EcdsaPublicKeyRequest' : HideAccountRequest } |
  { 'AddSignerRequest' : AddSignerRequest } |
  { 'HideAccountRequest' : HideAccountRequest } |
  { 'UpdateCanisterSettingsRequest' : UpdateCanisterSettingsRequest };
export interface Keys {
  'ecdsa' : [] | [Uint8Array | number[]],
  'addresses' : Array<[Chains, string]>,
  'identifier' : Uint8Array | number[],
}
export interface Ledger { 'keys' : Keys, 'subaccount' : Uint8Array | number[] }
export interface Outpoint { 'txid' : Uint8Array | number[], 'vout' : number }
export interface PendingRequest {
  'id' : bigint,
  'signers' : Array<Principal>,
  'request' : Request,
  'role' : Roles,
  'deadline' : bigint,
}
export interface RemoveSignerRequest { 'signer_id' : Principal }
export interface RenameAccountRequest {
  'account_id' : string,
  'new_name' : string,
}
export type Request = { 'BtcRequest' : BtcRequest } |
  { 'EvmRequest' : EvmRequest } |
  { 'IcpRequest' : IcpRequest } |
  { 'InnerRequest' : InnerRequest };
export type RequestStatus = { 'Fail' : null } |
  { 'Success' : null } |
  { 'Pending' : null };
export type Roles = { 'User' : null } |
  { 'Canister' : null } |
  { 'Admin' : null };
export interface Signer {
  'threshold' : [] | [number],
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'role' : Roles,
  'expires_at' : [] | [bigint],
}
export interface Tokens { 'e8s' : bigint }
export interface TopUpCanisterRequest {
  'fee' : [] | [Tokens],
  'account_id' : string,
  'canister_id' : [] | [Principal],
  'amount' : Tokens,
}
export interface UpdateCanisterSettingsRequest {
  'canister_id' : Principal,
  'settings' : CanisterSettings,
}
export interface UpdateSignerThresholdRequest {
  'threshold' : number,
  'signer_id' : Principal,
}
export interface UpgradeCanisterRequest {
  'wasm_version' : string,
  'wasm_hash' : Uint8Array | number[],
}
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : Outpoint,
}
export type UtxoFilter = { 'page' : Uint8Array | number[] } |
  { 'min_confirmations' : number };
export interface WalletAccount {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'ledger' : Ledger,
}
export interface WalletAccountView {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'addresses' : Array<[Chains, string]>,
  'environment' : Environment,
}
export interface WalletCanisterStatus {
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : AccountsCounter,
}
export interface _SERVICE {
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_generate_address' : ActorMethod<[string, Chains], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_icp_balance' : ActorMethod<[string, [] | [Principal]], Tokens>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_request_public_key' : ActorMethod<[string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'confirm_request' : ActorMethod<[bigint], ConfirmedRequest>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], AccountsCounter>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_accounts' : ActorMethod<[], Array<WalletAccount>>,
  'get_addresses' : ActorMethod<[string], Array<[Chains, string]>>,
  'get_confirmed' : ActorMethod<[bigint], ConfirmedRequest>,
  'get_confirmed_requests' : ActorMethod<[], Array<[bigint, ConfirmedRequest]>>,
  'get_requests' : ActorMethod<[], Array<PendingRequest>>,
  'get_signers' : ActorMethod<[], Array<[Principal, Signer]>>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'request_account_rename' : ActorMethod<
    [RenameAccountRequest, [] | [bigint]],
    bigint
  >,
  'request_balance_btc' : ActorMethod<
    [string, BtcNetwork, [] | [number]],
    bigint
  >,
  'request_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'request_btc_utxos' : ActorMethod<
    [string, BtcNetwork, [] | [UtxoFilter]],
    GetUtxosResponse
  >,
  'request_maker' : ActorMethod<[Request, [] | [bigint]], bigint>,
  'request_sign_message' : ActorMethod<
    [string, Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'request_sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Uint8Array | number[]
  >,
  'request_transfer_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    string
  >,
  'request_update_settings' : ActorMethod<
    [UpdateCanisterSettingsRequest, [] | [bigint]],
    bigint
  >,
  'reset_wallet' : ActorMethod<[], undefined>,
  'signer_add' : ActorMethod<[Principal, Roles], Array<[Principal, Signer]>>,
  'signer_remove' : ActorMethod<[Principal], Array<[Principal, Signer]>>,
  'status' : ActorMethod<[], WalletCanisterStatus>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'upgrage_wallet' : ActorMethod<[], undefined>,
  'version' : ActorMethod<[], string>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
