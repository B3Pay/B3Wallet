import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountsNonce {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface AddSigner {
  'threshold' : [] | [number],
  'name' : [] | [string],
  'role' : Roles,
  'signer_id' : Principal,
  'expires_at' : [] | [bigint],
}
export type BitcoinError = { 'InvalidAddress' : string } |
  { 'PublicKeyError' : string } |
  { 'NoUtxos' : null } |
  { 'InsufficientBalance' : [bigint, bigint] } |
  { 'InvalidFeePercentile' : string } |
  { 'GetBalance' : string } |
  { 'FeeTooHigh' : [bigint, bigint] } |
  { 'InvalidPublicKey' : string } |
  { 'GetFeeRate' : string } |
  { 'GetCurrentFeePercentiles' : string } |
  { 'InvalidChain' : string } |
  { 'Signature' : string } |
  { 'InvalidNetworkAddress' : string } |
  { 'GetUtxos' : string } |
  { 'SendRawTransaction' : string } |
  { 'SendTransaction' : string } |
  { 'SwapToCkbtc' : string };
export interface BtcChain {
  'pendings' : Array<BtcPending>,
  'subaccount' : Uint8Array | number[],
  'ecdsa_public_key' : Uint8Array | number[],
  'address' : string,
  'btc_network' : Minter,
  'min_confirmations' : [] | [number],
}
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface BtcPending { 'txid' : string, 'account' : string }
export interface BtcTransfer {
  'to' : string,
  'account_id' : string,
  'network' : Minter,
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
export type Chain = { 'EvmChain' : EvmChain } |
  { 'BtcChain' : BtcChain } |
  { 'IcpChain' : IcpChain } |
  { 'IcrcChain' : IcrcChain } |
  { 'CkbtcChain' : CkbtcChain };
export type ChainEnum = { 'BTC' : Minter } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'ICRC' : Principal } |
  { 'CKBTC' : Minter };
export interface CkbtcChain {
  'fee' : [] | [bigint],
  'pendings' : Array<CkbtcPending>,
  'memo' : [] | [Uint8Array | number[]],
  'minter' : Minter,
  'ledger' : Principal,
  'account' : ICRCAccount,
  'created_at_time' : [] | [bigint],
}
export type CkbtcError = { 'RetrieveBtcStatus' : RetrieveBtcStatus } |
  { 'MinterError' : MinterError } |
  { 'SendToInvalidAddress' : string } |
  { 'UpdateBalanceError' : UpdateBalanceError } |
  { 'IcrcError' : IcrcError } |
  { 'CkbtcGetBtcAddressError' : string } |
  { 'CkbtcSwapToBtcError' : string } |
  { 'ICRC1TransferError' : ICRC1TransferError };
export interface CkbtcPending { 'block_index' : bigint, 'txid' : [] | [bigint] }
export interface ConsentMessage {
  'title' : string,
  'message' : string,
  'reason' : string,
}
export interface CreateAccount {
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
export interface EvmChain {
  'pendings' : Array<EvmPending>,
  'chain_id' : bigint,
  'address' : string,
}
export interface EvmContractDeployed {
  'transaction' : EvmTransaction1559,
  'contract_address' : string,
}
export interface EvmDeployContract {
  'account_id' : string,
  'hex_byte_code' : Uint8Array | number[],
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
}
export type EvmError = { 'InvalidMessage' : string } |
  { 'InvalidAddress' : string } |
  { 'InvalidSignature' : string } |
  { 'InvalidPublicKey' : string } |
  { 'InvalidRecoveryId' : string } |
  { 'InvalidTransactionType' : null } |
  { 'NotSignedTransaction' : null };
export interface EvmPending { 'block_index' : bigint }
export interface EvmSignMessage {
  'account_id' : string,
  'chain_id' : bigint,
  'message' : Uint8Array | number[],
}
export interface EvmSignRawTransaction {
  'account_id' : string,
  'hex_raw_tx' : Uint8Array | number[],
  'chain_id' : bigint,
}
export interface EvmSignTranscation {
  'account_id' : string,
  'transaction' : EvmTransaction,
  'chain_id' : bigint,
}
export type EvmTransaction = { 'EvmTransaction1559' : EvmTransaction1559 } |
  { 'EvmTransaction2930' : EvmTransaction2930 } |
  { 'EvmTransactionLegacy' : EvmTransactionLegacy };
export interface EvmTransaction1559 {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'value' : bigint,
  'max_priority_fee_per_gas' : bigint,
  'data' : string,
  'max_fee_per_gas' : bigint,
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : bigint,
  'access_list' : Array<[string, Array<string>]>,
}
export interface EvmTransaction2930 {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'value' : bigint,
  'data' : string,
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : bigint,
  'access_list' : Array<[string, Array<string>]>,
  'gas_price' : bigint,
}
export interface EvmTransactionLegacy {
  'r' : string,
  's' : string,
  'v' : string,
  'to' : string,
  'value' : bigint,
  'data' : string,
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : bigint,
  'gas_price' : bigint,
}
export interface EvmTransfer {
  'to' : string,
  'account_id' : string,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
}
export interface EvmTransferErc20 {
  'to' : string,
  'account_id' : string,
  'value' : bigint,
  'max_priority_fee_per_gas' : [] | [bigint],
  'max_fee_per_gas' : [] | [bigint],
  'chain_id' : bigint,
  'nonce' : bigint,
  'gas_limit' : [] | [bigint],
  'contract_address' : string,
}
export type ExecutionResult = { 'AccountCreated' : CreateAccount } |
  { 'CanisterTopUped' : [NotifyTopUp, bigint] } |
  { 'BtcTransfered' : [BtcTransfer, string] } |
  { 'IcpTransfered' : [IcpTransfer, bigint] } |
  { 'AccountRenamed' : RenameAccount } |
  { 'EvmContractDeployed' : EvmContractDeployed } |
  { 'EvmErc20Transfered' : [EvmTransferErc20, string] } |
  { 'SignerRemoved' : RemoveSigner } |
  { 'EvmTransfered' : [EvmTransfer, string] } |
  { 'EvmRawTransactionSigned' : [EvmSignRawTransaction, string] } |
  { 'TopUpTransfered' : [TopUpTransfer, bigint] } |
  { 'SignerThresholdUpdated' : UpdateSignerThreshold } |
  { 'AccountHidden' : HideAccount } |
  { 'EvmMessageSigned' : [EvmSignMessage, Uint8Array | number[]] } |
  { 'CanisterSettingsUpdated' : UpdateCanisterSettings } |
  { 'SignerAdded' : AddSigner } |
  { 'CanisterUpgraded' : UpgradeCanister } |
  { 'EvmTransactionSigned' : [EvmSignTranscation, string] } |
  { 'AccountUnhidden' : HideAccount } |
  { 'AccountRemoved' : HideAccount };
export type HelperError = { 'CreateCanisterError' : string } |
  { 'UpdateCanisterControllersError' : string } |
  { 'ValidateSignerError' : string } |
  { 'InvalidSubaccount' : string } |
  { 'EncodeError' : string } |
  { 'InvalidAccountIdentifier' : null } |
  { 'RateLimitExceeded' : null } |
  { 'WasmHashError' : string } |
  { 'VersionError' : string } |
  { 'SignerNotAvailable' : null } |
  { 'InstallCodeError' : string } |
  { 'CanisterStatusError' : string } |
  { 'InvalidReleaseName' : string };
export interface HideAccount { 'account_id' : string }
export type ICRC1MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export type ICRC1TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface ICRCAccount {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type ICRCAccountError = { 'BadChecksum' : null } |
  { 'InvalidFormat' : null } |
  { 'InvalidSubaccount' : string } |
  { 'InvalidPrincipal' : string } |
  { 'Malformed' : string } |
  { 'NotCanonical' : null } |
  { 'HexDecode' : string };
export interface IcpChain {
  'fee' : Tokens,
  'pendings' : Array<IcpPending>,
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [Timestamp],
}
export type IcpError = { 'TransferError' : TransferError } |
  { 'TopUpPending' : string } |
  { 'CallError' : string } |
  { 'NotifyError' : NotifyError };
export interface IcpPending { 'block_index' : bigint, 'canister_id' : string }
export interface IcpTransfer {
  'to' : Uint8Array | number[],
  'fee' : [] | [Tokens],
  'account_id' : string,
  'memo' : [] | [bigint],
  'amount' : Tokens,
}
export interface IcrcChain {
  'fee' : [] | [bigint],
  'metadata' : Array<[string, ICRC1MetadataValue]>,
  'pendings' : Array<IcrcPending>,
  'memo' : [] | [Uint8Array | number[]],
  'canister_id' : Principal,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [bigint],
}
export type IcrcError = { 'UpdateBalanceError' : UpdateBalanceError } |
  { 'CallError' : string } |
  { 'ICRC1TransferError' : ICRC1TransferError };
export interface IcrcPending { 'tx_index' : bigint, 'block_index' : bigint }
export interface Ledger {
  'public_key' : [] | [Uint8Array | number[]],
  'subaccount' : Uint8Array | number[],
  'chains' : Array<[ChainEnum, Chain]>,
}
export type LedgerError = { 'InvalidMessageLength' : null } |
  { 'SignatureError' : string } |
  { 'ICRCAccountError' : ICRCAccountError } |
  { 'EvmError' : EvmError } |
  { 'IcpError' : IcpError } |
  { 'PublicKeyError' : string } |
  { 'UpdateBalanceError' : string } |
  { 'InvalidEcdsaPublicKey' : null } |
  { 'BitcoinError' : BitcoinError } |
  { 'GenerateError' : string } |
  { 'IcrcError' : IcrcError } |
  { 'MissingEcdsaPublicKey' : null } |
  { 'PendingIndexError' : bigint } |
  { 'MissingAddress' : null } |
  { 'BtcTxIdError' : string } |
  { 'CallError' : string } |
  { 'InvalidChain' : null } |
  { 'EcdsaPublicKeyError' : string } |
  { 'CkbtcError' : CkbtcError } |
  { 'EcdsaPublicKeyAlreadySet' : null };
export type Minter = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export type MinterError = { 'RetrieveBtcStatus' : RetrieveBtcStatus } |
  { 'UpdateBalanceError' : UpdateBalanceError } |
  { 'CallError' : string } |
  { 'RetrieveBtcError' : RetrieveBtcError } |
  { 'GetWithdrawalAccountError' : string } |
  { 'GetBtcAddressError' : string };
export type NotifyError = { 'TxTooOld' : { 'allowed_window_nanos' : bigint } } |
  { 'Refunded' : { 'block_index' : [] | [bigint], 'reason' : string } } |
  { 'InvalidTransaction' : string } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'TxDuplicate' : { 'duplicate_of' : bigint } } |
  { 'Other' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'Processing' : null } |
  { 'TxCreatedInFuture' : null } |
  { 'TransactionTooOld' : bigint } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export interface NotifyTopUp {
  'account_id' : string,
  'block_index' : bigint,
  'canister_id' : Principal,
}
export interface OutPoint { 'txid' : Uint8Array | number[], 'vout' : number }
export type PendingEnum = { 'IcrcPending' : IcrcPending } |
  { 'BtcPending' : BtcPending } |
  { 'CkbtcPending' : CkbtcPending } |
  { 'EvmPending' : EvmPending } |
  { 'IcpPending' : IcpPending };
export interface PendingRequest {
  'id' : bigint,
  'status' : RequestStatus,
  'responses' : Array<[Principal, Response]>,
  'request' : Request,
  'role' : Roles,
  'deadline' : bigint,
  'consent_message' : ConsentMessage,
  'created_at' : bigint,
  'version' : string,
}
export interface ProcessedRequest {
  'status' : RequestStatus,
  'result' : [] | [ExecutionResult],
  'method' : string,
  'request' : PendingRequest,
  'error' : [] | [string],
  'timestamp' : bigint,
}
export interface RemoveSigner { 'signer_id' : Principal }
export interface RenameAccount { 'account_id' : string, 'new_name' : string }
export type Request = { 'UnhideAccount' : HideAccount } |
  { 'EvmDeployContract' : EvmDeployContract } |
  { 'AddSigner' : AddSigner } |
  { 'IcpTransfer' : IcpTransfer } |
  { 'EvmSignRawTransaction' : EvmSignRawTransaction } |
  { 'EvmSignMessage' : EvmSignMessage } |
  { 'UpdateCanisterSettings' : UpdateCanisterSettings } |
  { 'RenameAccount' : RenameAccount } |
  { 'EvmSignTranscation' : EvmSignTranscation } |
  { 'EvmTransferErc20' : EvmTransferErc20 } |
  { 'HideAccount' : HideAccount } |
  { 'UpgradeCanister' : UpgradeCanister } |
  { 'TopUpTransfer' : TopUpTransfer } |
  { 'BtcTransfer' : BtcTransfer } |
  { 'RemoveAccount' : HideAccount } |
  { 'CreateAccount' : CreateAccount } |
  { 'EvmTransfer' : EvmTransfer } |
  { 'RemoveSigner' : RemoveSigner } |
  { 'UpdateSignerThreshold' : UpdateSignerThreshold };
export type RequestStatus = { 'Fail' : null } |
  { 'Success' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export type Response = { 'Reject' : null } |
  { 'Confirm' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<Principal> } |
  { 'Err' : WalletError };
export type RetrieveBtcError = { 'MalformedAddress' : string } |
  { 'GenericError' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  { 'AmountTooLow' : bigint } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export type RetrieveBtcStatus = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Pending' : null };
export type Roles = { 'User' : null } |
  { 'Canister' : null } |
  { 'Admin' : null } |
  { 'Threshold' : null };
export type SendResult = { 'BTC' : string } |
  { 'EVM' : null } |
  { 'ICP' : Result } |
  { 'ICRC' : bigint } |
  { 'CKBTC' : bigint };
export interface Signer {
  'threshold' : [] | [number],
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'role' : Roles,
  'expires_at' : [] | [bigint],
}
export interface Timestamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface TopUpTransfer {
  'fee' : [] | [Tokens],
  'account_id' : string,
  'canister_id' : Principal,
  'amount' : Tokens,
}
export type TransferError = {
    'TxTooOld' : { 'allowed_window_nanos' : bigint }
  } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'TxDuplicate' : { 'duplicate_of' : bigint } } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export type UpdateBalanceError = {
    'GenericError' : { 'error_message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  {
    'NoNewUtxos' : {
      'required_confirmations' : number,
      'current_confirmations' : [] | [number],
    }
  };
export interface UpdateCanisterSettings {
  'canister_id' : Principal,
  'settings' : CanisterSettings,
}
export interface UpdateSignerThreshold {
  'threshold' : number,
  'signer_id' : Principal,
}
export interface UpgradeCanister {
  'wasm_hash_string' : string,
  'wasm_version' : string,
}
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : OutPoint,
}
export type UtxoStatus = { 'ValueTooSmall' : Utxo } |
  { 'Tainted' : Utxo } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo,
    }
  } |
  { 'Checked' : Utxo };
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
  'pendings' : Array<PendingEnum>,
  'name' : string,
  'hidden' : boolean,
  'addresses' : Array<[ChainEnum, string]>,
  'environment' : Environment,
}
export interface WalletCanisterStatus {
  'name' : string,
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : AccountsNonce,
}
export type WalletError = { 'CannotRemoveDefaultAccount' : null } |
  { 'UpdateCanisterControllersError' : string } |
  { 'WalletAccountNotExists' : null } |
  { 'HelperError' : HelperError } |
  { 'WalletAlreadyInitialized' : null } |
  { 'EvmError' : EvmError } |
  { 'IcpError' : IcpError } |
  { 'UnknownError' : null } |
  { 'BitcoinError' : BitcoinError } |
  { 'IcrcError' : IcrcError } |
  { 'NotifyTopUpError' : string } |
  { 'ControllerAlreadyExists' : null } |
  { 'SignerAlreadyExists' : string } |
  { 'WalletAccountAlreadyExists' : null } |
  { 'SignerDoesNotExist' : string } |
  { 'LedgerError' : LedgerError } |
  { 'UpdateSettingsError' : string } |
  { 'WalletAccountCounterMismatch' : null } |
  { 'TooManyControllers' : null } |
  { 'CkbtcError' : CkbtcError } |
  { 'WasmNotLoaded' : null } |
  { 'ExecutionError' : string };
export interface WalletSettings {
  'controllers' : Array<Principal>,
  'initialised' : boolean,
  'metadata' : Array<[string, string]>,
}
export interface WalletSettingsAndSigners {
  'signers' : Array<[Principal, Signer]>,
  'settings' : WalletSettings,
}
export interface WasmDetails { 'hash' : Uint8Array | number[], 'size' : bigint }
export interface _SERVICE {
  'account_add_pending' : ActorMethod<
    [string, ChainEnum, PendingEnum],
    undefined
  >,
  'account_balance' : ActorMethod<[string, ChainEnum], bigint>,
  'account_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'account_check_pending' : ActorMethod<
    [string, ChainEnum, bigint],
    Array<UtxoStatus>
  >,
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_create_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_remove_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_remove_pending' : ActorMethod<
    [string, ChainEnum, bigint],
    undefined
  >,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send' : ActorMethod<[string, ChainEnum, string, bigint], SendResult>,
  'account_swap_btc_to_ckbtc' : ActorMethod<
    [string, BtcNetwork, bigint],
    BtcPending
  >,
  'account_swap_ckbtc_to_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal]],
    Result_1
  >,
  'add_controllers' : ActorMethod<[Principal], Result_2>,
  'add_setting' : ActorMethod<[string, string], undefined>,
  'canister_cycle_balance' : ActorMethod<[], bigint>,
  'canister_version' : ActorMethod<[], bigint>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], AccountsNonce>,
  'get_account_view' : ActorMethod<[string], WalletAccountView>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_addresses' : ActorMethod<[string], Array<[ChainEnum, string]>>,
  'get_pending_list' : ActorMethod<[], Array<PendingRequest>>,
  'get_processed' : ActorMethod<[bigint], ProcessedRequest>,
  'get_processed_list' : ActorMethod<[], Array<ProcessedRequest>>,
  'get_signers' : ActorMethod<[], Array<[Principal, Signer]>>,
  'init_wallet' : ActorMethod<
    [Array<Principal>, Array<Principal>, Array<[string, string]>],
    undefined
  >,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'name' : ActorMethod<[], string>,
  'remove_setting' : ActorMethod<[string], undefined>,
  'request_account_rename' : ActorMethod<
    [RenameAccount, string, [] | [bigint]],
    bigint
  >,
  'request_add_signer' : ActorMethod<
    [AddSigner, string, [] | [bigint]],
    bigint
  >,
  'request_create_account' : ActorMethod<
    [CreateAccount, string, [] | [bigint]],
    bigint
  >,
  'request_delete_account' : ActorMethod<
    [HideAccount, string, [] | [bigint]],
    bigint
  >,
  'request_maker' : ActorMethod<[Request, string, [] | [bigint]], bigint>,
  'request_sign_message' : ActorMethod<
    [string, Uint8Array | number[]],
    Uint8Array | number[]
  >,
  'request_sign_transaction' : ActorMethod<
    [string, Uint8Array | number[], bigint],
    Uint8Array | number[]
  >,
  'request_transfer_btc' : ActorMethod<
    [BtcTransfer, string, [] | [bigint]],
    bigint
  >,
  'request_transfer_icp' : ActorMethod<
    [IcpTransfer, string, [] | [bigint]],
    bigint
  >,
  'request_update_settings' : ActorMethod<
    [UpdateCanisterSettings, string, [] | [bigint]],
    bigint
  >,
  'reset_wallet' : ActorMethod<[], undefined>,
  'response' : ActorMethod<[bigint, Response], ProcessedRequest>,
  'retrieve_btc_status' : ActorMethod<[Minter, bigint], RetrieveBtcStatus>,
  'setting_and_signer' : ActorMethod<[], WalletSettingsAndSigners>,
  'signer_add' : ActorMethod<[Principal, Roles], Array<[Principal, Signer]>>,
  'signer_remove' : ActorMethod<[Principal], Array<[Principal, Signer]>>,
  'status' : ActorMethod<[], WalletCanisterStatus>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'update_controllers' : ActorMethod<[Array<Principal>], Result_2>,
  'upgrage_wallet' : ActorMethod<[], undefined>,
  'validate_signer' : ActorMethod<[Principal], boolean>,
  'version' : ActorMethod<[], string>,
  'wasm_details' : ActorMethod<[], WasmDetails>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
