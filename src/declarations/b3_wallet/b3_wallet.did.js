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
  const Roles = IDL.Variant({
    'Operator' : IDL.Null,
    'User' : IDL.Null,
    'Canister' : IDL.Null,
    'Admin' : IDL.Null,
    'Owner' : IDL.Null,
  });
  const Signer = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Opt(IDL.Text),
    'role' : Roles,
    'expires_at' : IDL.Opt(IDL.Nat64),
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
  const EvmSignRequest = IDL.Record({
    'id' : IDL.Nat64,
    'transaction' : EvmTransaction,
    'deadline' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const SendIcpRequest = IDL.Record({
    'id' : IDL.Nat64,
    'to' : IDL.Principal,
    'deadline' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const SendBitcoinRequest = IDL.Record({
    'id' : IDL.Nat64,
    'deadline' : IDL.Nat64,
    'address' : IDL.Text,
    'amount' : IDL.Nat64,
  });
  const AddSignerRequest = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'role' : IDL.Text,
    'canister_id' : IDL.Principal,
    'deadline' : IDL.Nat64,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const CallRequest = IDL.Record({
    'id' : IDL.Nat64,
    'arg' : IDL.Vec(IDL.Nat8),
    'canister_id' : IDL.Principal,
    'deadline' : IDL.Nat64,
    'sender' : IDL.Opt(IDL.Principal),
    'cycles' : IDL.Opt(IDL.Nat64),
    'method_name' : IDL.Text,
  });
  const TopUpCanisterRequest = IDL.Record({
    'id' : IDL.Nat64,
    'canister_id' : IDL.Principal,
    'deadline' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const CanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const UpdateSettingsArgument = IDL.Record({
    'canister_id' : IDL.Principal,
    'settings' : CanisterSettings,
  });
  const UpdateSettingsRequest = IDL.Record({
    'id' : IDL.Nat64,
    'deadline' : IDL.Nat64,
    'settings' : UpdateSettingsArgument,
  });
  const UpgradeCanisterRequest = IDL.Record({
    'id' : IDL.Nat64,
    'wasm_hash_string' : IDL.Text,
    'wasm_version' : IDL.Text,
    'deadline' : IDL.Nat64,
    'wasm_hash' : IDL.Vec(IDL.Nat8),
  });
  const RawRandRequest = IDL.Record({
    'id' : IDL.Nat64,
    'deadline' : IDL.Nat64,
    'length' : IDL.Nat32,
  });
  const QueryRequest = IDL.Record({
    'id' : IDL.Nat64,
    'arg' : IDL.Vec(IDL.Nat8),
    'canister_id' : IDL.Principal,
    'deadline' : IDL.Nat64,
    'sender' : IDL.Opt(IDL.Principal),
    'method_name' : IDL.Text,
  });
  const InnerCanisterRequest = IDL.Variant({
    'AddSigner' : AddSignerRequest,
    'Call' : CallRequest,
    'TopUpCanister' : TopUpCanisterRequest,
    'UpdateSettings' : UpdateSettingsRequest,
    'UpdateCanister' : UpgradeCanisterRequest,
    'RawRand' : RawRandRequest,
    'Query' : QueryRequest,
  });
  const SignRequest = IDL.Variant({
    'Evm' : EvmSignRequest,
    'Icp' : SendIcpRequest,
    'Bitcoin' : SendBitcoinRequest,
    'InnerCanister' : InnerCanisterRequest,
  });
  const SignedTransaction = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const AccountsCounters = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const State = IDL.Record({
    'confirms' : IDL.Vec(IDL.Tuple(IDL.Nat64, SignedTransaction)),
    'accounts' : IDL.Vec(IDL.Tuple(IDL.Text, WalletAccount)),
    'counters' : AccountsCounters,
    'requests' : IDL.Vec(SignRequest),
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
  const AccountsStatus = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const SignerCanisterStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AccountsStatus,
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
        [IDL.Vec(IDL.Nat8)],
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
    'add_signer' : IDL.Func(
        [IDL.Principal, Roles],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        [],
      ),
    'get_account' : IDL.Func([IDL.Text], [WalletAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(WalletAccount)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'get_sign_requests' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [SignRequest],
        ['query'],
      ),
    'get_signed_transaction' : IDL.Func(
        [IDL.Nat64],
        [SignedTransaction],
        ['query'],
      ),
    'get_signers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        ['query'],
      ),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'reintall_canister' : IDL.Func([], [], []),
    'remove_signer' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Signer))],
        [],
      ),
    'request_sign_message' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Nat8)],
        [],
      ),
    'request_sign_transaction' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64],
        [SignedTransaction],
        [],
      ),
    'reset_accounts' : IDL.Func([], [State], []),
    'status' : IDL.Func([], [SignerCanisterStatus], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'update_canister_controllers' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'upgrade_canister' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
