export const idlFactory = ({ IDL }) => {
  const HideAccount = IDL.Record({ 'account_id' : IDL.Text });
  const EvmDeployContract = IDL.Record({
    'account_id' : IDL.Text,
    'hex_byte_code' : IDL.Vec(IDL.Nat8),
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
  });
  const Roles = IDL.Variant({
    'User' : IDL.Null,
    'Canister' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const AddSigner = IDL.Record({
    'threshold' : IDL.Opt(IDL.Nat8),
    'name' : IDL.Text,
    'role' : Roles,
    'signer_id' : IDL.Principal,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const IcpTransfer = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'fee' : IDL.Opt(Tokens),
    'account_id' : IDL.Text,
    'memo' : IDL.Opt(IDL.Nat64),
    'amount' : Tokens,
  });
  const EvmSignRawTransaction = IDL.Record({
    'account_id' : IDL.Text,
    'hex_raw_tx' : IDL.Vec(IDL.Nat8),
    'chain_id' : IDL.Nat64,
  });
  const EvmSignMessage = IDL.Record({
    'account_id' : IDL.Text,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const CanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const UpdateCanisterSettings = IDL.Record({
    'canister_id' : IDL.Principal,
    'settings' : CanisterSettings,
  });
  const RenameAccount = IDL.Record({
    'account_id' : IDL.Text,
    'new_name' : IDL.Text,
  });
  const EvmTransaction1559 = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Nat64,
    'data' : IDL.Text,
    'max_fee_per_gas' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Nat64,
    'access_list' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text))),
  });
  const EvmTransaction2930 = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'value' : IDL.Nat64,
    'data' : IDL.Text,
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Nat64,
    'access_list' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text))),
    'gas_price' : IDL.Nat64,
  });
  const EvmTransactionLegacy = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'value' : IDL.Nat64,
    'data' : IDL.Text,
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Nat64,
    'gas_price' : IDL.Nat64,
  });
  const EvmTransaction = IDL.Variant({
    'EvmTransaction1559' : EvmTransaction1559,
    'EvmTransaction2930' : EvmTransaction2930,
    'EvmTransactionLegacy' : EvmTransactionLegacy,
  });
  const EvmSignTranscation = IDL.Record({
    'account_id' : IDL.Text,
    'transaction' : EvmTransaction,
    'chain_id' : IDL.Nat64,
  });
  const EvmTransferErc20 = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
    'contract_address' : IDL.Text,
  });
  const BtcNetwork = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const ChainEnum = IDL.Variant({
    'BTC' : BtcNetwork,
    'EVM' : IDL.Nat64,
    'ICP' : IDL.Null,
    'ICRC' : IDL.Principal,
    'CKBTC' : BtcNetwork,
  });
  const SendToken = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'chain' : ChainEnum,
    'amount' : IDL.Nat64,
  });
  const UpgradeCanister = IDL.Record({
    'wasm_hash_string' : IDL.Text,
    'wasm_version' : IDL.Text,
  });
  const TopUpTransfer = IDL.Record({
    'fee' : IDL.Opt(Tokens),
    'account_id' : IDL.Text,
    'canister_id' : IDL.Principal,
    'amount' : Tokens,
  });
  const BtcTransfer = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'network' : BtcNetwork,
    'amount' : IDL.Nat64,
  });
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const CreateAccount = IDL.Record({
    'env' : IDL.Opt(Environment),
    'name' : IDL.Opt(IDL.Text),
  });
  const EvmTransfer = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
  });
  const RemoveSigner = IDL.Record({ 'signer_id' : IDL.Principal });
  const UpdateSignerThreshold = IDL.Record({
    'threshold' : IDL.Nat8,
    'signer_id' : IDL.Principal,
  });
  const Request = IDL.Variant({
    'UnhideAccount' : HideAccount,
    'EvmDeployContract' : EvmDeployContract,
    'AddSigner' : AddSigner,
    'IcpTransfer' : IcpTransfer,
    'EvmSignRawTransaction' : EvmSignRawTransaction,
    'EvmSignMessage' : EvmSignMessage,
    'UpdateCanisterSettings' : UpdateCanisterSettings,
    'RenameAccount' : RenameAccount,
    'EvmSignTranscation' : EvmSignTranscation,
    'EvmTransferErc20' : EvmTransferErc20,
    'SendToken' : SendToken,
    'HideAccount' : HideAccount,
    'UpgradeCanister' : UpgradeCanister,
    'TopUpTransfer' : TopUpTransfer,
    'BtcTransfer' : BtcTransfer,
    'RemoveAccount' : HideAccount,
    'CreateAccount' : CreateAccount,
    'EvmTransfer' : EvmTransfer,
    'RemoveSigner' : RemoveSigner,
    'UpdateSignerThreshold' : UpdateSignerThreshold,
  });
  return IDL.Service({
    'request_connect' : IDL.Func([IDL.Principal], [IDL.Nat64], []),
    'request_maker' : IDL.Func(
        [IDL.Principal, Request, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
