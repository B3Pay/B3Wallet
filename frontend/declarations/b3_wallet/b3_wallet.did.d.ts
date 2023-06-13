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
export interface BtcChain { 'address' : string, 'btc_network' : Minter }
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
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
  'memo' : [] | [Uint8Array | number[]],
  'minter' : Minter,
  'ledger' : Principal,
  'account' : ICRCAccount,
  'created_at_time' : [] | [bigint],
}
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
export interface EvmChain { 'chain_id' : bigint, 'address' : string }
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
  { 'CanisterTopUped' : [TopUpCanister, bigint] } |
  { 'BtcTransfered' : [BtcTransfer, string] } |
  { 'IcpTransfered' : [IcpTransfer, bigint] } |
  { 'AccountRenamed' : RenameAccount } |
  { 'EvmContractDeployed' : EvmContractDeployed } |
  { 'EvmErc20Transfered' : [EvmTransferErc20, string] } |
  { 'SignerRemoved' : RemoveSigner } |
  { 'EvmTransfered' : [EvmTransfer, string] } |
  { 'EvmRawTransactionSigned' : [EvmSignRawTransaction, string] } |
  { 'SignerThresholdUpdated' : UpdateSignerThreshold } |
  { 'AccountHidden' : HideAccount } |
  { 'EvmMessageSigned' : [EvmSignMessage, Uint8Array | number[]] } |
  { 'CanisterSettingsUpdated' : UpdateCanisterSettings } |
  { 'SignerAdded' : AddSigner } |
  { 'CanisterUpgraded' : UpgradeCanister } |
  { 'EvmTransactionSigned' : [EvmSignTranscation, string] } |
  { 'AccountUnhidden' : HideAccount } |
  { 'AccountRemoved' : HideAccount };
export interface GetUtxosResponse {
  'next_page' : [] | [Uint8Array | number[]],
  'tip_height' : number,
  'tip_block_hash' : Uint8Array | number[],
  'utxos' : Array<Utxo>,
}
export interface HideAccount { 'account_id' : string }
export type ICRC1MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface ICRCAccount {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface IcpChain {
  'fee' : Tokens,
  'memo' : bigint,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [Timestamp],
}
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
  'memo' : [] | [Uint8Array | number[]],
  'canister_id' : Principal,
  'subaccount' : Uint8Array | number[],
  'created_at_time' : [] | [bigint],
}
export interface Ledger {
  'public_key' : [] | [Uint8Array | number[]],
  'pending_sends' : Array<[Minter, Array<[bigint, RetrieveBtcStatus]>]>,
  'subaccount' : Uint8Array | number[],
  'pending_receives' : Array<[Minter, string]>,
  'chains' : Array<[ChainEnum, Chain]>,
}
export type Minter = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface OutPoint { 'txid' : Uint8Array | number[], 'vout' : number }
export interface Outpoint { 'txid' : Uint8Array | number[], 'vout' : number }
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
  { 'TopUpCanister' : TopUpCanister } |
  { 'EvmSignTranscation' : EvmSignTranscation } |
  { 'EvmTransferErc20' : EvmTransferErc20 } |
  { 'HideAccount' : HideAccount } |
  { 'UpgradeCanister' : UpgradeCanister } |
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
export interface Signer {
  'threshold' : [] | [number],
  'metadata' : Array<[string, string]>,
  'name' : [] | [string],
  'role' : Roles,
  'expires_at' : [] | [bigint],
}
export interface Timestamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface TopUpCanister {
  'fee' : [] | [Tokens],
  'account_id' : string,
  'canister_id' : Principal,
  'amount' : Tokens,
}
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
  'outpoint' : Outpoint,
}
export type UtxoFilter = { 'page' : Uint8Array | number[] } |
  { 'min_confirmations' : number };
export type UtxoStatus = { 'ValueTooSmall' : Utxo_1 } |
  { 'Tainted' : Utxo_1 } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo_1,
    }
  } |
  { 'Checked' : Utxo_1 };
export interface Utxo_1 {
  'height' : number,
  'value' : bigint,
  'outpoint' : OutPoint,
}
export interface WalletAccount {
  'id' : string,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'ledger' : Ledger,
}
export interface WalletAccountView {
  'id' : string,
  'pending_send' : Array<[Minter, Array<[bigint, RetrieveBtcStatus]>]>,
  'metadata' : Array<[string, string]>,
  'name' : string,
  'hidden' : boolean,
  'pending_receive' : Array<[Minter, string]>,
  'addresses' : Array<[ChainEnum, string]>,
  'environment' : Environment,
}
export interface WalletCanisterStatus {
  'canister_id' : Principal,
  'status_at' : bigint,
  'version' : string,
  'canister_status' : CanisterStatusResponse,
  'account_status' : AccountsNonce,
}
export interface _SERVICE {
  'account_balance' : ActorMethod<[string, ChainEnum], bigint>,
  'account_balance_btc' : ActorMethod<
    [string, BtcNetwork, [] | [number]],
    bigint
  >,
  'account_btc_fees' : ActorMethod<[BtcNetwork, number], bigint>,
  'account_btc_utxos' : ActorMethod<
    [string, BtcNetwork, [] | [UtxoFilter]],
    GetUtxosResponse
  >,
  'account_ckbtc_balance' : ActorMethod<[string, BtcNetwork], bigint>,
  'account_create' : ActorMethod<
    [[] | [Environment], [] | [string]],
    undefined
  >,
  'account_create_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_hide' : ActorMethod<[string], undefined>,
  'account_icp_balance' : ActorMethod<[string], bigint>,
  'account_icrc_balance' : ActorMethod<[string, Principal], bigint>,
  'account_remove' : ActorMethod<[string], undefined>,
  'account_remove_address' : ActorMethod<[string, ChainEnum], undefined>,
  'account_remove_pending_receive' : ActorMethod<
    [string, BtcNetwork],
    undefined
  >,
  'account_remove_pending_send' : ActorMethod<
    [string, BtcNetwork, bigint],
    undefined
  >,
  'account_rename' : ActorMethod<[string, string], undefined>,
  'account_restore' : ActorMethod<[Environment, bigint], undefined>,
  'account_send' : ActorMethod<[string, ChainEnum, string, bigint], undefined>,
  'account_send_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    string
  >,
  'account_send_icp' : ActorMethod<
    [string, string, Tokens, [] | [Tokens], [] | [bigint]],
    bigint
  >,
  'account_swap_btc_to_ckbtc' : ActorMethod<
    [string, BtcNetwork, bigint],
    string
  >,
  'account_swap_ckbtc_to_btc' : ActorMethod<
    [string, BtcNetwork, string, bigint],
    string
  >,
  'account_top_up_and_notify' : ActorMethod<
    [string, Tokens, [] | [Principal], [] | [Tokens]],
    bigint
  >,
  'account_update_balance' : ActorMethod<
    [string, BtcNetwork],
    Array<UtxoStatus>
  >,
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
  'load_wasm' : ActorMethod<[Uint8Array | number[]], bigint>,
  'name' : ActorMethod<[], string>,
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
