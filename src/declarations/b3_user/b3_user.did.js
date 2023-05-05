export const idlFactory = ({ IDL }) => {
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const BtcAddresses = IDL.Record({
    'mainnet' : IDL.Text,
    'testnet' : IDL.Text,
  });
  const Addresses = IDL.Record({ 'btc' : BtcAddresses, 'eth' : IDL.Text });
  const Keys = IDL.Record({
    'addresses' : Addresses,
    'bytes' : IDL.Vec(IDL.Nat8),
  });
  const Ecdsa = IDL.Record({ 'env' : Environment, 'path' : IDL.Vec(IDL.Nat8) });
  const Allowance = IDL.Record({
    'updated_at' : IDL.Nat64,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : IDL.Nat64,
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const TransactionType = IDL.Variant({
    'EIP1559' : IDL.Null,
    'EIP2930' : IDL.Null,
    'Legacy' : IDL.Null,
  });
  const Transaction = IDL.Record({
    'r' : IDL.Text,
    's' : IDL.Text,
    'v' : IDL.Text,
    'to' : IDL.Text,
    'transaction_type' : TransactionType,
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
  const SignRequest = IDL.Record({
    'transaction' : Transaction,
    'deadline' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const SignedTransaction = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const Account = IDL.Record({
    'id' : IDL.Text,
    'keys' : Keys,
    'name' : IDL.Text,
    'ecdsa' : Ecdsa,
    'canisters' : IDL.Vec(IDL.Tuple(IDL.Principal, Allowance)),
    'requests' : IDL.Vec(IDL.Tuple(IDL.Principal, SignRequest)),
    'signed' : SignedTransaction,
  });
  const Result = IDL.Variant({ 'Ok' : Account, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : SignRequest, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, Allowance)),
    'Err' : IDL.Text,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const SetAllowance = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({ 'Ok' : SignedTransaction, 'Err' : IDL.Text });
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
    'id' : IDL.Principal,
    'status' : CanisterStatusResponse,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
  });
  const Result_7 = IDL.Variant({ 'Ok' : CanisterStatus, 'Err' : IDL.Text });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result_8 = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  return IDL.Service({
    'change_owner' : IDL.Func([IDL.Principal], [], []),
    'create_account' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [Result],
        [],
      ),
    'get_account' : IDL.Func([IDL.Text], [Account], ['query']),
    'get_account_requests' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_1],
        ['query'],
      ),
    'get_accounts' : IDL.Func([], [IDL.Vec(Account)], ['query']),
    'get_addresses' : IDL.Func([IDL.Text], [Addresses], ['query']),
    'get_caller' : IDL.Func([], [IDL.Principal], ['query']),
    'get_connected_canisters' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_owner' : IDL.Func([], [IDL.Principal], ['query']),
    'get_signed' : IDL.Func([IDL.Text], [SignedTransaction], ['query']),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Text], [Result_3], []),
    'number_of_accounts' : IDL.Func([], [IDL.Nat8], ['query']),
    'reintall_canister' : IDL.Func([], [], []),
    'request_allowance' : IDL.Func(
        [IDL.Text, IDL.Principal, SetAllowance],
        [Result_4],
        [],
      ),
    'request_sign' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64],
        [SignRequest],
        [],
      ),
    'reset_user' : IDL.Func([], [], []),
    'reset_wasm' : IDL.Func([], [], []),
    'sign_message' : IDL.Func([IDL.Text, IDL.Vec(IDL.Nat8)], [Result_5], []),
    'sign_transaction' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64],
        [Result_6],
        [],
      ),
    'status' : IDL.Func([], [Result_7], []),
    'update_canister_controllers' : IDL.Func(
        [IDL.Vec(IDL.Principal)],
        [Result_8],
        [],
      ),
    'upgrade_canister' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_version' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
