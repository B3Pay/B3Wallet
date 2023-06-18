import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddSigner {
  'threshold' : [] | [number],
  'name' : string,
  'role' : Roles,
  'signer_id' : Principal,
  'expires_at' : [] | [bigint],
}
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export interface BtcTransfer {
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
export type ChainEnum = { 'BTC' : BtcNetwork } |
  { 'EVM' : bigint } |
  { 'ICP' : null } |
  { 'ICRC' : Principal } |
  { 'CKBTC' : BtcNetwork };
export interface CreateAccount {
  'env' : [] | [Environment],
  'name' : [] | [string],
}
export type Environment = { 'Production' : null } |
  { 'Development' : null } |
  { 'Staging' : null };
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
export interface HideAccount { 'account_id' : string }
export interface IcpTransfer {
  'to' : Uint8Array | number[],
  'fee' : [] | [Tokens],
  'account_id' : string,
  'memo' : [] | [bigint],
  'amount' : Tokens,
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
  { 'SendToken' : SendToken } |
  { 'HideAccount' : HideAccount } |
  { 'UpgradeCanister' : UpgradeCanister } |
  { 'TopUpTransfer' : TopUpTransfer } |
  { 'BtcTransfer' : BtcTransfer } |
  { 'RemoveAccount' : HideAccount } |
  { 'CreateAccount' : CreateAccount } |
  { 'EvmTransfer' : EvmTransfer } |
  { 'RemoveSigner' : RemoveSigner } |
  { 'UpdateSignerThreshold' : UpdateSignerThreshold };
export type Roles = { 'User' : null } |
  { 'Canister' : null } |
  { 'Admin' : null };
export interface SendToken {
  'to' : string,
  'account_id' : string,
  'chain' : ChainEnum,
  'amount' : bigint,
}
export interface Tokens { 'e8s' : bigint }
export interface TopUpTransfer {
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
export interface _SERVICE {
  'request_connect' : ActorMethod<[Principal], bigint>,
  'request_maker' : ActorMethod<
    [Principal, Request, string, [] | [bigint]],
    bigint
  >,
}
