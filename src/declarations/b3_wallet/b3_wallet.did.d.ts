import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type AccessLevel = { 'ReadOnly' : null } |
  { 'Limited' : Array<OperationAccess> } |
  { 'Canister' : null } |
  { 'FullAccess' : null };
export interface AddUser {
  'threshold' : [] | [number],
  'name' : string,
  'role' : Role,
  'signer_id' : Uint8Array | number[],
  'expires_at' : [] | [bigint],
}
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
  'amount' : TokenAmount,
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
  'query_stats' : QueryStats,
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
export interface HideAccount { 'account_id' : string }
export interface ICPToken { 'e8s' : bigint }
export interface ICPTransferTimestamp { 'timestamp_nanos' : bigint }
export type ICRC1MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface ICRCAccount {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface IcpChain {
  'fee' : ICPToken,
  'pendings' : Array<IcpPending>,
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [ICPTransferTimestamp],
}
export interface IcpPending { 'block_index' : bigint, 'canister_id' : string }
export interface IcpTransfer {
  'to' : Uint8Array | number[],
  'fee' : [] | [ICPToken],
  'account_id' : string,
  'memo' : [] | [bigint],
  'amount' : ICPToken,
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
export interface IcrcPending { 'tx_index' : bigint, 'block_index' : bigint }
export interface Ledger {
  'public_key' : [] | [Uint8Array | number[]],
  'subaccount' : Uint8Array | number[],
  'chains' : Array<[ChainEnum, Chain]>,
}
export interface LogEntry {
  'counter' : bigint,
  'file' : string,
  'line' : number,
  'cycle' : [] | [bigint],
  'version' : string,
  'message' : string,
  'timestamp' : bigint,
  'variant' : LogVariant,
}
export type LogVariant = { 'info' : null } |
  { 'warn' : null } |
  { 'error' : null };
export type Minter = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface NotifyTopUp {
  'account_id' : string,
  'block_index' : bigint,
  'canister_id' : Principal,
}
export type Operation = { 'UnhideAccount' : HideAccount } |
  { 'EvmDeployContract' : EvmDeployContract } |
  { 'IcpTransfer' : IcpTransfer } |
  { 'EvmSignRawTransaction' : EvmSignRawTransaction } |
  { 'EvmSignMessage' : EvmSignMessage } |
  { 'UpdateCanisterSettings' : UpdateCanisterSettings } |
  { 'RenameAccount' : RenameAccount } |
  { 'AddUser' : AddUser } |
  { 'EvmSignTranscation' : EvmSignTranscation } |
  { 'EvmTransferErc20' : EvmTransferErc20 } |
  { 'SendToken' : SendToken } |
  { 'HideAccount' : HideAccount } |
  { 'UpgradeCanister' : UpgradeCanister } |
  { 'TopUpTransfer' : TopUpTransfer } |
  { 'BtcTransfer' : BtcTransfer } |
  { 'RemoveUser' : RemoveUser } |
  { 'RemoveAccount' : HideAccount } |
  { 'CreateAccount' : CreateAccount } |
  { 'EvmTransfer' : EvmTransfer };
export interface OperationAccess {
  'valid_until' : [] | [bigint],
  'operation' : OperationEnum,
}
export type OperationEnum = { 'UnhideAccount' : null } |
  { 'EvmDeployContract' : null } |
  { 'IcpTransfer' : null } |
  { 'EvmSignRawTransaction' : null } |
  { 'EvmSignMessage' : null } |
  { 'UpdateCanisterSettings' : null } |
  { 'RenameAccount' : null } |
  { 'AddUser' : null } |
  { 'EvmSignTranscation' : null } |
  { 'EvmTransferErc20' : null } |
  { 'SendToken' : null } |
  { 'HideAccount' : null } |
  { 'UpgradeCanister' : null } |
  { 'TopUpTransfer' : null } |
  { 'BtcTransfer' : null } |
  { 'RemoveUser' : null } |
  { 'RemoveAccount' : null } |
  { 'CreateAccount' : null } |
  { 'EvmTransfer' : null };
export type OperationResult = { 'Empty' : null } |
  { 'AccountCreated' : CreateAccount } |
  { 'CanisterTopUped' : [NotifyTopUp, bigint] } |
  { 'BtcTransfered' : [BtcTransfer, string] } |
  { 'IcpTransfered' : [IcpTransfer, bigint] } |
  { 'TokenSent' : [SendToken, SendResult] } |
  { 'AccountRenamed' : RenameAccount } |
  { 'EvmContractDeployed' : EvmContractDeployed } |
  { 'EvmErc20Transfered' : [EvmTransferErc20, string] } |
  { 'SignerRemoved' : RemoveUser } |
  { 'EvmTransfered' : [EvmTransfer, string] } |
  { 'EvmRawTransactionSigned' : [EvmSignRawTransaction, string] } |
  { 'TopUpTransfered' : [TopUpTransfer, bigint] } |
  { 'AccountHidden' : HideAccount } |
  { 'EvmMessageSigned' : [EvmSignMessage, Uint8Array | number[]] } |
  { 'CanisterSettingsUpdated' : UpdateCanisterSettings } |
  { 'SignerAdded' : AddUser } |
  { 'CanisterUpgraded' : UpgradeCanister } |
  { 'EvmTransactionSigned' : [EvmSignTranscation, string] } |
  { 'AccountUnhidden' : HideAccount } |
  { 'AccountRemoved' : HideAccount };
export type OperationStatus = { 'Fail' : null } |
  { 'Success' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export interface OutPoint { 'txid' : Uint8Array | number[], 'vout' : number }
export type PendingEnum = { 'BTC' : BtcPending } |
  { 'EVM' : EvmPending } |
  { 'ICP' : IcpPending } |
  { 'ICRC' : IcrcPending } |
  { 'CKBTC' : CkbtcPending };
export interface PendingOperation {
  'id' : bigint,
  'status' : OperationStatus,
  'responses' : Array<[Uint8Array | number[], Response]>,
  'allowed_signers' : Array<Uint8Array | number[]>,
  'request' : Operation,
  'deadline' : bigint,
  'consent_message' : ConsentMessage,
  'created_at' : bigint,
  'created_by' : Uint8Array | number[],
  'version' : string,
}
export interface ProcessedOperation {
  'status' : OperationStatus,
  'result' : OperationResult,
  'method' : string,
  'error' : [] | [string],
  'operation' : PendingOperation,
  'timestamp' : bigint,
}
export interface QueryStats {
  'response_payload_bytes_total' : bigint,
  'num_instructions_total' : bigint,
  'num_calls_total' : bigint,
  'request_payload_bytes_total' : bigint,
}
export interface RemoveUser { 'signer_id' : Uint8Array | number[] }
export interface RenameAccount { 'account_id' : string, 'new_name' : string }
export type Response = { 'Reject' : null } |
  { 'Confirm' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : ProcessedOperation } |
  { 'Err' : string };
export type RetrieveBtcStatus = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Pending' : null };
export interface Role { 'access_level' : AccessLevel, 'name' : string }
export type SendResult = { 'BTC' : string } |
  { 'EVM' : null } |
  { 'ICP' : bigint } |
  { 'ICRC' : bigint } |
  { 'CKBTC' : bigint };
export interface SendToken {
  'to' : string,
  'account_id' : string,
  'chain' : ChainEnum,
  'amount' : TokenAmount,
}
export interface TokenAmount { 'decimals' : number, 'amount' : bigint }
export interface TopUpTransfer {
  'fee' : [] | [ICPToken],
  'account_id' : string,
  'canister_id' : Principal,
  'amount' : ICPToken,
}
export interface UpdateCanisterSettings {
  'canister_id' : Principal,
  'settings' : CanisterSettings,
}
export interface UpgradeCanister {
  'wasm_hash_string' : string,
  'wasm_version' : string,
}
export interface User {
  'metadata' : Array<[string, string]>,
  'name' : string,
  'role' : Role,
  'expires_at' : [] | [bigint],
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
export interface WalletAccountsNonce {
  'staging' : bigint,
  'production' : bigint,
  'development' : bigint,
}
export interface WalletCanisterStatus {
  'name' : string,
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : WalletAccountsNonce,
}
export interface WalletController {
  'metadata' : Array<[string, string]>,
  'name' : string,
}
export interface WalletInititializeArgs {
  'controllers' : Array<[Principal, WalletController]>,
  'metadata' : [] | [Array<[string, string]>],
}
export interface WalletSettings {
  'freezing_threshold' : [] | [bigint],
  'controllers' : Array<[Principal, WalletController]>,
  'initialised' : boolean,
  'metadata' : Array<[string, string]>,
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
}
export interface WalletSettingsAndSigners {
  'signers' : Array<[Uint8Array | number[], User]>,
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
  'account_check_pending' : ActorMethod<[string, ChainEnum, bigint], undefined>,
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
  'account_send' : ActorMethod<
    [string, ChainEnum, string, TokenAmount],
    SendResult
  >,
  'account_swap_btc_to_ckbtc' : ActorMethod<
    [string, BtcNetwork, bigint],
    BtcPending
  >,
  'account_swap_ckbtc_to_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    bigint
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, ICPToken, [] | [Principal]],
    Result
  >,
  'account_update_balance' : ActorMethod<
    [string, BtcNetwork],
    Array<UtxoStatus>
  >,
  'add_controller_and_update' : ActorMethod<
    [Principal, string, [] | [Array<[string, string]>]],
    undefined
  >,
  'add_setting_metadata' : ActorMethod<[string, string], undefined>,
  'canister_cycle_balance' : ActorMethod<[], bigint>,
  'canister_version' : ActorMethod<[], bigint>,
  'get_account' : ActorMethod<[string], WalletAccount>,
  'get_account_count' : ActorMethod<[], bigint>,
  'get_account_counters' : ActorMethod<[], WalletAccountsNonce>,
  'get_account_view' : ActorMethod<[string], WalletAccountView>,
  'get_account_views' : ActorMethod<[], Array<WalletAccountView>>,
  'get_addresses' : ActorMethod<[string], Array<[ChainEnum, string]>>,
  'get_pending_list' : ActorMethod<[], Array<PendingOperation>>,
  'get_processed_list' : ActorMethod<[], Array<ProcessedOperation>>,
  'get_roles' : ActorMethod<[], Array<[bigint, Role]>>,
  'get_signers' : ActorMethod<[], Array<[Uint8Array | number[], User]>>,
  'init_wallet' : ActorMethod<[WalletInititializeArgs], undefined>,
  'is_connected' : ActorMethod<[], boolean>,
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'name' : ActorMethod<[], string>,
  'print_log_entries' : ActorMethod<[], Array<LogEntry>>,
  'refresh_settings' : ActorMethod<[], undefined>,
  'remove_setting_metadata' : ActorMethod<[string], undefined>,
  'report_bug' : ActorMethod<[Principal, string], undefined>,
  'request_account_rename' : ActorMethod<
    [RenameAccount, string, [] | [bigint]],
    bigint
  >,
  'request_add_signer' : ActorMethod<[AddUser, string, [] | [bigint]], bigint>,
  'request_connect' : ActorMethod<[string], bigint>,
  'request_create_account' : ActorMethod<
    [CreateAccount, string, [] | [bigint]],
    bigint
  >,
  'request_delete_account' : ActorMethod<
    [HideAccount, string, [] | [bigint]],
    bigint
  >,
  'request_maker' : ActorMethod<[Operation, string, [] | [bigint]], bigint>,
  'request_remove_signer' : ActorMethod<
    [RemoveUser, string, [] | [bigint]],
    bigint
  >,
  'request_send' : ActorMethod<[SendToken, string, [] | [bigint]], bigint>,
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
  'request_upgrade_canister' : ActorMethod<[string], bigint>,
  'reset_accounts' : ActorMethod<[], undefined>,
  'response' : ActorMethod<[bigint, Response], Result_1>,
  'retrieve_btc_status' : ActorMethod<[Minter, bigint], RetrieveBtcStatus>,
  'role_add' : ActorMethod<[Role], Array<[bigint, Role]>>,
  'role_remove' : ActorMethod<[bigint], Array<[bigint, Role]>>,
  'setting_and_signer' : ActorMethod<[], WalletSettingsAndSigners>,
  'signer_add' : ActorMethod<
    [Uint8Array | number[], Role],
    Array<[Uint8Array | number[], User]>
  >,
  'signer_remove' : ActorMethod<
    [Uint8Array | number[]],
    Array<[Uint8Array | number[], User]>
  >,
  'status' : ActorMethod<[], WalletCanisterStatus>,
  'uninstall_wallet' : ActorMethod<[], undefined>,
  'unload_wasm' : ActorMethod<[], bigint>,
  'update_controller' : ActorMethod<
    [Array<[Principal, WalletController]>],
    Array<[Principal, WalletController]>
  >,
  'update_settings' : ActorMethod<[], undefined>,
  'upgrage_wallet' : ActorMethod<[], undefined>,
  'validate_signer' : ActorMethod<[Uint8Array | number[]], boolean>,
  'version' : ActorMethod<[], string>,
  'wasm_details' : ActorMethod<[], WasmDetails>,
  'wasm_hash' : ActorMethod<[], Uint8Array | number[]>,
  'wasm_hash_string' : ActorMethod<[], string>,
}
