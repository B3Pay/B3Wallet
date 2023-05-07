export const idlFactory = ({ IDL }) => {
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Allowance = IDL.Record({
    'updated_at' : IDL.Nat64,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : IDL.Nat64,
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, Allowance)),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const SetAllowance = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Nat8),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
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
  const SignRequest = IDL.Record({
    'transaction' : EvmTransaction,
    'deadline' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'message' : IDL.Vec(IDL.Nat8),
  });
  const Result_4 = IDL.Variant({
    'Ok' : SignRequest,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const SignedTransaction = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const Result_5 = IDL.Variant({
    'Ok' : SignedTransaction,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const NotifyError = IDL.Variant({
    'Refunded' : IDL.Record({
      'block_index' : IDL.Opt(IDL.Nat64),
      'reason' : IDL.Text,
    }),
    'InvalidTransaction' : IDL.Text,
    'Other' : IDL.Record({
      'error_message' : IDL.Text,
      'error_code' : IDL.Nat64,
    }),
    'Processing' : IDL.Null,
    'TransactionTooOld' : IDL.Nat64,
  });
  const NotifyTopUpResult = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : NotifyError,
  });
  const Result_6 = IDL.Variant({
    'Ok' : NotifyTopUpResult,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_7 = IDL.Variant({
    'Ok' : IDL.Principal,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
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
  const Account = IDL.Record({
    'id' : IDL.Text,
    'name' : IDL.Text,
    'ledger' : Ledger,
    'canisters' : IDL.Vec(IDL.Tuple(IDL.Principal, Allowance)),
    'requests' : IDL.Vec(IDL.Tuple(IDL.Principal, SignRequest)),
    'signed' : SignedTransaction,
  });
  const Result_8 = IDL.Variant({
    'Ok' : Account,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
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
  const Result_9 = IDL.Variant({
    'Ok' : IDL.Text,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_10 = IDL.Variant({
    'Ok' : IDL.Nat64,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_11 = IDL.Variant({
    'Ok' : Tokens,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const TransferError = IDL.Variant({
    'TxTooOld' : IDL.Record({ 'allowed_window_nanos' : IDL.Nat64 }),
    'BadFee' : IDL.Record({ 'expected_fee' : Tokens }),
    'TxDuplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat64 }),
    'TxCreatedInFuture' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : Tokens }),
  });
  const Result_12 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : TransferError });
  const Result_13 = IDL.Variant({
    'Ok' : Result_12,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
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
    'id' : IDL.Principal,
    'status' : CanisterStatusResponse,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
  });
  const SignerError = IDL.Variant({
    'InvalidMessageLength' : IDL.Null,
    'InvalidAddress' : IDL.Null,
    'MaximumDevelopmentAccountsReached' : IDL.Null,
    'CyclesMintingError' : IDL.Text,
    'MaximumAccountsReached' : IDL.Null,
    'InvalidTx' : IDL.Text,
    'AccountNotExists' : IDL.Null,
    'RequestNotExists' : IDL.Null,
    'PublicKeyError' : IDL.Text,
    'PublicKeyAlreadyExists' : IDL.Null,
    'UnknownError' : IDL.Null,
    'InvalidEcdsaPublicKey' : IDL.Null,
    'GenerateError' : IDL.Text,
    'InvalidSignature' : IDL.Text,
    'MissingEcdsaPublicKey' : IDL.Null,
    'InvalidMsg' : IDL.Text,
    'LedgerError' : IDL.Text,
    'InvalidAccountIdentifier' : IDL.Null,
    'SignError' : IDL.Text,
    'CallerIsNotOwner' : IDL.Null,
    'CanisterStatusError' : IDL.Text,
    'MaximumProductionAccountsReached' : IDL.Null,
  });
  const Result_14 = IDL.Variant({ 'Ok' : CanisterStatus, 'Err' : SignerError });
  return IDL.Service({
    'account_addresses' : IDL.Func([IDL.Text], [Result], ['query']),
    'account_connected_canisters' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'account_request_allowance' : IDL.Func(
        [IDL.Text, IDL.Principal, SetAllowance],
        [Result_2],
        [],
      ),
    'account_request_public_key' : IDL.Func([IDL.Text], [Result_3], []),
    'account_sign_requests' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_4],
        ['query'],
      ),
    'account_signed_transaction' : IDL.Func([IDL.Text], [Result_5], ['query']),
    'account_topup_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal), IDL.Opt(Tokens)],
        [Result_6],
        [],
      ),
    'change_owner' : IDL.Func([IDL.Principal], [Result_7], []),
    'create_account' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [Result_8],
        [],
      ),
    'generate_account_address' : IDL.Func([IDL.Text, Network], [Result_9], []),
    'get_account' : IDL.Func([IDL.Text], [Result_8], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(Account)], ['query']),
    'get_caller' : IDL.Func([], [IDL.Principal], ['query']),
    'get_number_of_accounts' : IDL.Func([], [IDL.Nat8], ['query']),
    'get_owner' : IDL.Func([], [IDL.Principal], ['query']),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Text], [Result_10], []),
    'reintall_canister' : IDL.Func([], [], []),
    'rename_account' : IDL.Func([IDL.Text, IDL.Text], [Result_9], []),
    'request_account_balance' : IDL.Func([IDL.Text], [Result_11], []),
    'request_account_sign_message' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result_3],
        [],
      ),
    'request_account_sign_transaction' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64],
        [Result_5],
        [],
      ),
    'request_account_transfer_icp' : IDL.Func(
        [IDL.Text, Tokens, IDL.Text, IDL.Opt(Tokens), IDL.Opt(IDL.Nat64)],
        [Result_13],
        [],
      ),
    'reset_accounts' : IDL.Func([], [], []),
    'reset_wasm' : IDL.Func([], [], []),
    'status' : IDL.Func([], [Result_14], []),
    'update_canister_controllers' : IDL.Func(
        [IDL.Vec(IDL.Principal)],
        [Result_2],
        [],
      ),
    'upgrade_canister' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_version' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
