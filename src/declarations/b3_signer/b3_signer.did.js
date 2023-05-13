export const idlFactory = ({ IDL }) => {
  const AccountsStatus = IDL.Record({
    'stag_counter' : IDL.Nat64,
    'prod_counter' : IDL.Nat64,
    'dev_counter' : IDL.Nat64,
  });
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
  const SignerAllowance = IDL.Record({
    'updated_at' : IDL.Nat64,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : IDL.Nat64,
    'limit' : IDL.Opt(IDL.Nat8),
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
    'transaction' : EvmTransaction,
    'deadline' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const SignedTransaction = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const SignerAccount = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'ledger' : Ledger,
    'canisters' : IDL.Vec(IDL.Tuple(IDL.Principal, SignerAllowance)),
    'requests' : IDL.Vec(IDL.Tuple(IDL.Principal, EvmSignRequest)),
    'signed' : SignedTransaction,
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
  const SignerAllowanceArgs = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const State = IDL.Record({
    'stag_counter' : IDL.Nat64,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'prod_counter' : IDL.Nat64,
    'accounts' : IDL.Vec(IDL.Tuple(IDL.Text, SignerAccount)),
    'dev_counter' : IDL.Nat64,
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
  const CanisterStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_counter' : IDL.Nat64,
  });
  return IDL.Service({
    'account_status' : IDL.Func([], [AccountsStatus], ['query']),
    'change_owner' : IDL.Func([IDL.Principal], [IDL.Principal], []),
    'create_account' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [SignerAccount],
        [],
      ),
    'generate_address' : IDL.Func([IDL.Text, Network], [IDL.Text], []),
    'get_account' : IDL.Func([IDL.Text], [SignerAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(SignerAccount)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'get_connected_canisters' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Principal, SignerAllowance))],
        ['query'],
      ),
    'get_owner' : IDL.Func([], [IDL.Principal], ['query']),
    'get_sign_requests' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [EvmSignRequest],
        ['query'],
      ),
    'get_signed_transaction' : IDL.Func(
        [IDL.Text],
        [SignedTransaction],
        ['query'],
      ),
    'hide_account' : IDL.Func([IDL.Text], [], []),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'reintall_canister' : IDL.Func([], [], []),
    'remove_account' : IDL.Func([IDL.Text], [], []),
    'rename_account' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'request_allowance' : IDL.Func(
        [IDL.Text, IDL.Principal, SignerAllowanceArgs],
        [],
        [],
      ),
    'request_balance' : IDL.Func([IDL.Text], [Tokens], []),
    'request_public_key' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Nat8)], []),
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
    'send_icp' : IDL.Func(
        [IDL.Text, IDL.Text, Tokens, IDL.Opt(Tokens), IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'status' : IDL.Func([], [CanisterStatus], []),
    'top_up_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal), IDL.Opt(Tokens)],
        [IDL.Nat],
        [],
      ),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'update_canister_controllers' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'upgrade_canister' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
