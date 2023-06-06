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
export interface BTC { 'address' : string, 'btc_network' : BtcNetwork }
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export type BtcRequest = { 'BtcTransferRequest' : BtcTransferRequest };
export interface BtcTransferRequest {
  'to' : string,
  'account_id' : string,
  'network' : BtcNetwork,
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
export type Chain = { 'BTC' : BTC } |
  { 'EVM' : EVM } |
  { 'ICP' : ICP } |
  { 'ICRC' : ICRC };
export type ChainType = { 'BTC' : BtcNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'ICRC' : Principal };
export interface ConsendInfo { 'consent_message' : string, 'language' : string }
export interface ConsentMessageRequest {
  'arg' : RequestArgs,
  'method' : string,
  'consent_preferences' : ConsentPreferences,
}
export type ConsentMessageResponse = { 'MalformedCall' : ErrorInfo } |
  { 'Valid' : ConsendInfo } |
  { 'Other' : string } |
  { 'Forbidden' : ErrorInfo };
export interface ConsentPreferences { 'language' : string }
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
export interface EVM { 'chain_id' : bigint, 'address' : string }
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
export interface ICP {
  'fee' : Tokens,
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [Timestamp],
}
export interface ICRC {
  'fee' : bigint,
  'memo' : [] | [Uint8Array | number[]],
  'canister_id' : Principal,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [bigint],
}
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
export interface Ledger {
  'subaccount' : Uint8Array | number[],
  'ecdsa' : [] | [Uint8Array | number[]],
  'chains' : Array<[ChainType, Chain]>,
}
export interface Outpoint { 'txid' : Uint8Array | number[], 'vout' : number }
export interface PendingRequest {
  'id' : bigint,
  'request' : Request,
  'role' : Roles,
  'deadline' : bigint,
  'consent_message' : ConsentMessageRequest,
  'response' : Array<[Principal, RequestResponse]>,
}
export interface ProcessedRequest {
  'status' : RequestStatus,
  'method' : string,
  'request' : PendingRequest,
  'error' : [] | [RequestError],
  'message' : ConsentMessageResponse,
  'timestamp' : bigint,
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
export interface RequestArgs {
  'request' : Request,
  'role' : Roles,
  'deadline' : [] | [bigint],
}
export type RequestError = { 'InvalidMessage' : string } |
  { 'InvalidMessageLength' : null } |
  { 'RequestAlreadySigned' : string } |
  { 'InvalidAddress' : null } |
  { 'CannotRemoveDefaultAccount' : null } |
  { 'RequestNotProcessed' : bigint } |
  { 'DeadlineExceeded' : null } |
  { 'InvalidController' : null } |
  { 'WalletAccountNotExists' : null } |
  { 'InvalidEvmTransactionType' : null } |
  { 'CyclesMintingError' : string } |
  { 'InvalidTx' : string } |
  { 'SignerRoleNotAuthorized' : string } |
  { 'RequestNotExists' : null } |
  { 'BitcoinGetBalanceError' : string } |
  { 'BitcoinInsufficientBalanceError' : [bigint, bigint] } |
  { 'PublicKeyError' : string } |
  { 'RequestExpired' : null } |
  { 'NoUtxos' : null } |
  { 'UnknownError' : null } |
  { 'InvalidEcdsaPublicKey' : null } |
  { 'GenerateError' : string } |
  { 'InvalidTransaction' : string } |
  { 'InvalidSignature' : string } |
  { 'SignerRoleNotFound' : [string, string] } |
  { 'NotifyTopUpError' : string } |
  { 'MissingEcdsaPublicKey' : null } |
  { 'InvalidMsg' : string } |
  { 'SignerAlreadyExists' : string } |
  { 'BitcoinGetFeeRateError' : string } |
  { 'MissingSighashType' : null } |
  { 'WalletAccountAlreadyExists' : null } |
  { 'BitcoinGetUtxosError' : string } |
  { 'MissingAddress' : null } |
  { 'SignerDoesNotExist' : string } |
  { 'LedgerError' : string } |
  { 'RecoverableSignatureError' : string } |
  { 'InvalidAccountIdentifier' : null } |
  { 'RequestAlreadyProcessed' : bigint } |
  { 'InvalidPublicKey' : string } |
  { 'UpdateSettingsError' : string } |
  { 'SignError' : string } |
  { 'RequestNotFound' : bigint } |
  { 'BitcoinFeeTooHighError' : [bigint, bigint] } |
  { 'WalletAccountCounterMismatch' : null } |
  { 'BitcoinGetAddressError' : null } |
  { 'InvalidRequest' : null } |
  { 'CallerIsNotOwner' : null } |
  { 'RequestRejected' : null } |
  { 'InvalidRecoveryId' : string } |
  { 'BitcoinInvalidFeePercentile' : null } |
  { 'InvalidNetwork' : null } |
  { 'BitcoinSignatureError' : string } |
  { 'InvalidNetworkAddress' : null } |
  { 'MissingWitnessScript' : null } |
  { 'SignerNotFound' : string } |
  { 'BitcoinGetCurrentFeePercentilesError' : string } |
  { 'Processing' : null } |
  { 'BitcoinSendTransactionError' : string } |
  { 'NotSignedTransaction' : null } |
  { 'ExecutionError' : string } |
  { 'TransactionTooOld' : bigint } |
  { 'CanisterStatusError' : string } |
  { 'EcdsaPublicKeyAlreadySet' : null } |
  { 'BitcoinSendRawTransactionError' : string };
export type RequestResponse = { 'Reject' : null } |
  { 'Confirm' : null };
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
export interface Timestamp { 'timestamp_nanos' : bigint }
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
  'wasm_hash_string' : string,
  'wasm_version' : string,
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
  'addresses' : Array<[ChainType, string]>,
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
  'account_balance_btc' : ActorMethod<
    [string, BtcNetwork, [] | [number]],
    bigint
  >,
  'account_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'account_btc_utxos' : ActorMethod<
    [string, BtcNetwork, [] | [UtxoFilter]],
    GetUtxosResponse
  >,
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_generate_address' : ActorMethod<[string, ChainType], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_icp_balance' : ActorMethod<[string], bigint>,
  'account_icrc_balance' : ActorMethod<[string, Principal], bigint>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_remove_address' : ActorMethod<[string, ChainType], undefined>,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_request_public_key' : ActorMethod<[string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send' : ActorMethod<[string, ChainType, string, bigint], undefined>,
  'account_send_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    string
  >,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], AccountsCounter>,
  'get_account_view' : ActorMethod<[string], WalletAccountView>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_addresses' : ActorMethod<[string], Array<[ChainType, string]>>,
  'get_pending_list' : ActorMethod<[], Array<PendingRequest>>,
  'get_processed' : ActorMethod<[bigint], ProcessedRequest>,
  'get_processed_list' : ActorMethod<[], Array<ProcessedRequest>>,
  'get_signers' : ActorMethod<[], Array<[Principal, Signer]>>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'request_account_rename' : ActorMethod<
    [RenameAccountRequest, [] | [bigint]],
    bigint
  >,
  'request_add_signer' : ActorMethod<[AddSignerRequest, [] | [bigint]], bigint>,
  'request_create_account' : ActorMethod<
    [CreateAccountRequest, [] | [bigint]],
    bigint
  >,
  'request_delete_account' : ActorMethod<
    [HideAccountRequest, [] | [bigint]],
    bigint
  >,
  'request_maker' : ActorMethod<[Request, [] | [bigint]], bigint>,
  'request_response' : ActorMethod<[bigint, RequestResponse], ProcessedRequest>,
  'request_sign_message' : ActorMethod<
    [string, Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'request_sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Uint8Array | number[]
  >,
  'request_transfer_btc' : ActorMethod<
    [BtcTransferRequest, [] | [bigint]],
    bigint
  >,
  'request_transfer_icp' : ActorMethod<
    [IcpTransferRequest, [] | [bigint]],
    bigint
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
  'validate_signer' : ActorMethod<[Principal], boolean>,
  'version' : ActorMethod<[], string>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
