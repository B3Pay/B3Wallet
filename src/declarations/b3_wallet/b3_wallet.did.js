export const idlFactory = ({ IDL }) => {
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const PublicKeys = IDL.Record({
    'ecdsa' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'addresses' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'identifier' : IDL.Vec(IDL.Nat8),
  });
  const Ledger = IDL.Record({
    'subaccount' : IDL.Vec(IDL.Nat8),
    'public_keys' : PublicKeys,
  });
  const WalletAccount = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'ledger' : Ledger,
  });
  const BitcoinNetwork = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const Network = IDL.Variant({
    'BTC' : BitcoinNetwork,
    'EVM' : IDL.Nat64,
    'ICP' : IDL.Null,
    'SNS' : IDL.Text,
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const RequestStatus = IDL.Variant({
    'Fail' : IDL.Null,
    'Success' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const BtcTransferRequest = IDL.Record({
    'deadline' : IDL.Nat64,
    'address' : IDL.Text,
    'amount' : IDL.Nat64,
  });
  const BtcRequest = IDL.Variant({ 'BtcTransferRequest' : BtcTransferRequest });
  const EvmDeployContractRequest = IDL.Record({
    'account_id' : IDL.Text,
    'hex_byte_code' : IDL.Vec(IDL.Nat8),
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
  });
  const EvmSignRawTransactionRequest = IDL.Record({
    'account_id' : IDL.Text,
    'hex_raw_tx' : IDL.Vec(IDL.Nat8),
    'chain_id' : IDL.Nat64,
  });
  const EvmSignMessageRequest = IDL.Record({
    'account_id' : IDL.Text,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const EvmTransferErc20Request = IDL.Record({
    'account_id' : IDL.Text,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'address' : IDL.Text,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
    'contract_address' : IDL.Text,
  });
  const EvmTransactionType = IDL.Variant({
    'EIP1559' : IDL.Null,
    'EIP2930' : IDL.Null,
    'Legacy' : IDL.Null,
  });
  const EvmTransaction = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'transaction_type' : EvmTransactionType,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'data' : IDL.Text,
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Nat64,
    'access_list' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text)))),
    'gas_price' : IDL.Opt(IDL.Nat64),
  });
  const EvmSignTranscationRequest = IDL.Record({
    'account_id' : IDL.Text,
    'transaction' : EvmTransaction,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const EvmTransferEthRequest = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'value' : IDL.Nat64,
    'max_priority_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'max_fee_per_gas' : IDL.Opt(IDL.Nat64),
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
    'gas_limit' : IDL.Opt(IDL.Nat64),
  });
  const EvmRequest = IDL.Variant({
    'EvmDeployContractRequest' : EvmDeployContractRequest,
    'EvmSignRawTransactionRequest' : EvmSignRawTransactionRequest,
    'EvmSignMessageRequest' : EvmSignMessageRequest,
    'EvmTransferErc20Request' : EvmTransferErc20Request,
    'EvmSignTranscationRequest' : EvmSignTranscationRequest,
    'EvmTransferEthRequest' : EvmTransferEthRequest,
  });
  const IcpTransferRequest = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'fee' : IDL.Opt(Tokens),
    'account_id' : IDL.Text,
    'memo' : IDL.Opt(IDL.Nat64),
    'amount' : Tokens,
  });
  const TopUpCanisterRequest = IDL.Record({
    'fee' : IDL.Opt(Tokens),
    'account_id' : IDL.Text,
    'canister_id' : IDL.Opt(IDL.Principal),
    'amount' : Tokens,
  });
  const IcpRequest = IDL.Variant({
    'IcpTransferRequest' : IcpTransferRequest,
    'TopUpCanisterRequest' : TopUpCanisterRequest,
  });
  const UpgradeCanisterRequest = IDL.Record({
    'wasm_hash_string' : IDL.Text,
    'wasm_version' : IDL.Text,
    'wasm_hash' : IDL.Vec(IDL.Nat8),
  });
  const RenameAccountRequest = IDL.Record({
    'account_id' : IDL.Text,
    'new_name' : IDL.Text,
  });
  const HideAccountRequest = IDL.Record({ 'account_id' : IDL.Text });
  const CreateAccountRequest = IDL.Record({
    'env' : IDL.Opt(Environment),
    'name' : IDL.Opt(IDL.Text),
  });
  const RemoveSignerRequest = IDL.Record({ 'signer_id' : IDL.Principal });
  const UpdateSignerThresholdRequest = IDL.Record({
    'threshold' : IDL.Nat8,
    'signer_id' : IDL.Principal,
  });
  const Roles = IDL.Variant({
    'User' : IDL.Null,
    'Canister' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const AddSignerRequest = IDL.Record({
    'threshold' : IDL.Opt(IDL.Nat8),
    'name' : IDL.Opt(IDL.Text),
    'role' : Roles,
    'signer_id' : IDL.Principal,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const CanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const UpdateCanisterSettingsRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'settings' : CanisterSettings,
  });
  const InnerRequest = IDL.Variant({
    'UpgradeCanisterRequest' : UpgradeCanisterRequest,
    'RenameAccountRequest' : RenameAccountRequest,
    'UnhideAccountRequest' : HideAccountRequest,
    'CreateAccountRequest' : CreateAccountRequest,
    'RemoveAccountRequest' : HideAccountRequest,
    'RemoveSignerRequest' : RemoveSignerRequest,
    'UpdateSignerThresholdRequest' : UpdateSignerThresholdRequest,
    'EcdsaPublicKeyRequest' : HideAccountRequest,
    'AddSignerRequest' : AddSignerRequest,
    'HideAccountRequest' : HideAccountRequest,
    'UpdateCanisterSettingsRequest' : UpdateCanisterSettingsRequest,
  });
  const Request = IDL.Variant({
    'BtcRequest' : BtcRequest,
    'EvmRequest' : EvmRequest,
    'IcpRequest' : IcpRequest,
    'InnerRequest' : InnerRequest,
  });
  const PendingRequest = IDL.Record({
    'id' : IDL.Nat64,
    'signers' : IDL.Vec(IDL.Principal),
    'request' : Request,
    'role' : Roles,
    'deadline' : IDL.Nat64,
  });
  const ConfirmedRequest = IDL.Record({
    'status' : RequestStatus,
    'request' : PendingRequest,
    'error' : IDL.Text,
    'message' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const Signer = IDL.Record({
    'threshold' : IDL.Opt(IDL.Nat8),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Opt(IDL.Text),
    'role' : Roles,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const CanisterStatusType = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatusResponse = IDL.Record({
    'status' : CanisterStatusType,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const AccountsCounter = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const SignerCanisterStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AccountsCounter,
  });
  return IDL.Service({
    'account_create' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [WalletAccount],
        [],
      ),
    'account_generate_address' : IDL.Func([IDL.Text, Network], [IDL.Text], []),
    'account_hide' : IDL.Func([IDL.Text], [], []),
    'account_icp_balance' : IDL.Func([IDL.Text], [Tokens], []),
    'account_remove' : IDL.Func([IDL.Text], [], []),
    'account_rename' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'account_request_public_key' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        [],
      ),
    'account_send_icp' : IDL.Func(
        [IDL.Text, IDL.Text, Tokens, IDL.Opt(Tokens), IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'account_top_up_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal), IDL.Opt(Tokens)],
        [IDL.Nat],
        [],
      ),
    'confirm_request' : IDL.Func([IDL.Nat64], [ConfirmedRequest], []),
    'get_account' : IDL.Func([IDL.Text], [WalletAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(WalletAccount)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'get_confirmed' : IDL.Func([IDL.Nat64], [ConfirmedRequest], ['query']),
    'get_confirmed_requests' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, ConfirmedRequest))],
        ['query'],
      ),
    'get_requests' : IDL.Func([], [IDL.Vec(PendingRequest)], ['query']),
    'get_signers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        ['query'],
      ),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'request_account_rename' : IDL.Func(
        [RenameAccountRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_maker' : IDL.Func([Request, IDL.Opt(IDL.Nat64)], [IDL.Nat64], []),
    'request_sign_message' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'request_sign_transaction' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'request_update_settings' : IDL.Func(
        [UpdateCanisterSettingsRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'reset_wallet' : IDL.Func([], [], []),
    'signer_add' : IDL.Func(
        [IDL.Principal, Roles],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        [],
      ),
    'signer_remove' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        [],
      ),
    'status' : IDL.Func([], [SignerCanisterStatus], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
