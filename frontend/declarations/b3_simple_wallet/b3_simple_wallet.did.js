export const idlFactory = ({ IDL }) => {
  const Minter = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const ChainEnum = IDL.Variant({
    'BTC' : Minter,
    'EVM' : IDL.Nat64,
    'ICP' : IDL.Null,
    'ICRC' : IDL.Principal,
    'CKBTC' : Minter,
  });
  const IcrcPending = IDL.Record({
    'tx_index' : IDL.Nat,
    'block_index' : IDL.Nat64,
  });
  const BtcPending = IDL.Record({ 'txid' : IDL.Text, 'account' : IDL.Text });
  const CkbtcPending = IDL.Record({
    'block_index' : IDL.Nat64,
    'txid' : IDL.Opt(IDL.Nat),
  });
  const EvmPending = IDL.Record({ 'block_index' : IDL.Nat64 });
  const IcpPending = IDL.Record({
    'block_index' : IDL.Nat64,
    'canister_id' : IDL.Text,
  });
  const PendingEnum = IDL.Variant({
    'IcrcPending' : IcrcPending,
    'BtcPending' : BtcPending,
    'CkbtcPending' : CkbtcPending,
    'EvmPending' : EvmPending,
    'IcpPending' : IcpPending,
  });
  const BtcNetwork = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const OutPoint = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
  });
  const Utxo = IDL.Record({
    'height' : IDL.Nat32,
    'value' : IDL.Nat64,
    'outpoint' : OutPoint,
  });
  const UtxoStatus = IDL.Variant({
    'ValueTooSmall' : Utxo,
    'Tainted' : Utxo,
    'Minted' : IDL.Record({
      'minted_amount' : IDL.Nat64,
      'block_index' : IDL.Nat64,
      'utxo' : Utxo,
    }),
    'Checked' : Utxo,
  });
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const TransferError = IDL.Variant({
    'TxTooOld' : IDL.Record({ 'allowed_window_nanos' : IDL.Nat64 }),
    'BadFee' : IDL.Record({ 'expected_fee' : Tokens }),
    'TxDuplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat64 }),
    'TxCreatedInFuture' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : Tokens }),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : TransferError });
  const SendResult = IDL.Variant({
    'BTC' : IDL.Text,
    'EVM' : IDL.Null,
    'ICP' : Result,
    'ICRC' : IDL.Nat,
    'CKBTC' : IDL.Nat,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const EvmChain = IDL.Record({
    'pendings' : IDL.Vec(EvmPending),
    'chain_id' : IDL.Nat64,
    'address' : IDL.Text,
  });
  const BtcChain = IDL.Record({
    'pendings' : IDL.Vec(BtcPending),
    'subaccount' : IDL.Vec(IDL.Nat8),
    'ecdsa_public_key' : IDL.Vec(IDL.Nat8),
    'address' : IDL.Text,
    'btc_network' : Minter,
    'min_confirmations' : IDL.Opt(IDL.Nat32),
  });
  const Timestamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const IcpChain = IDL.Record({
    'fee' : Tokens,
    'pendings' : IDL.Vec(IcpPending),
    'memo' : IDL.Nat64,
    'subaccount' : IDL.Vec(IDL.Nat8),
    'created_at_time' : IDL.Opt(Timestamp),
  });
  const ICRC1MetadataValue = IDL.Variant({
    'Int' : IDL.Int,
    'Nat' : IDL.Nat,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
  });
  const IcrcChain = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, ICRC1MetadataValue)),
    'pendings' : IDL.Vec(IcrcPending),
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'canister_id' : IDL.Principal,
    'subaccount' : IDL.Vec(IDL.Nat8),
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const ICRCAccount = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CkbtcChain = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'pendings' : IDL.Vec(CkbtcPending),
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'minter' : Minter,
    'ledger' : IDL.Principal,
    'account' : ICRCAccount,
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const Chain = IDL.Variant({
    'EvmChain' : EvmChain,
    'BtcChain' : BtcChain,
    'IcpChain' : IcpChain,
    'IcrcChain' : IcrcChain,
    'CkbtcChain' : CkbtcChain,
  });
  const Ledger = IDL.Record({
    'public_key' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'subaccount' : IDL.Vec(IDL.Nat8),
    'chains' : IDL.Vec(IDL.Tuple(ChainEnum, Chain)),
  });
  const WalletAccount = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'ledger' : Ledger,
  });
  const AccountsNonce = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const WalletAccountView = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'pendings' : IDL.Vec(PendingEnum),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'addresses' : IDL.Vec(IDL.Tuple(ChainEnum, IDL.Text)),
    'environment' : Environment,
  });
  const Controller = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
  });
  const InititializeWalletArgs = IDL.Record({
    'controllers' : IDL.Vec(IDL.Tuple(IDL.Principal, Controller)),
    'metadata' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const RetrieveBtcStatus = IDL.Variant({
    'Signing' : IDL.Null,
    'Confirmed' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'Sending' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'AmountTooLow' : IDL.Null,
    'Unknown' : IDL.Null,
    'Submitted' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'Pending' : IDL.Null,
  });
  const WalletSettings = IDL.Record({
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Vec(IDL.Tuple(IDL.Principal, Controller)),
    'initialised' : IDL.Bool,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
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
  const WalletCanisterStatus = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AccountsNonce,
  });
  const WasmDetails = IDL.Record({
    'hash' : IDL.Vec(IDL.Nat8),
    'size' : IDL.Nat64,
  });
  return IDL.Service({
    'account_add_pending' : IDL.Func(
        [IDL.Text, ChainEnum, PendingEnum],
        [],
        [],
      ),
    'account_balance' : IDL.Func([IDL.Text, ChainEnum], [IDL.Nat], []),
    'account_btc_fees' : IDL.Func([BtcNetwork, IDL.Nat8], [IDL.Nat64], []),
    'account_check_pending' : IDL.Func(
        [IDL.Text, ChainEnum, IDL.Nat64],
        [IDL.Vec(UtxoStatus)],
        [],
      ),
    'account_create' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [],
        [],
      ),
    'account_create_address' : IDL.Func([IDL.Text, ChainEnum], [], []),
    'account_hide' : IDL.Func([IDL.Text], [], []),
    'account_remove' : IDL.Func([IDL.Text], [], []),
    'account_remove_address' : IDL.Func([IDL.Text, ChainEnum], [], []),
    'account_remove_pending' : IDL.Func(
        [IDL.Text, ChainEnum, IDL.Nat64],
        [],
        [],
      ),
    'account_rename' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'account_restore' : IDL.Func([Environment, IDL.Nat64], [], []),
    'account_send' : IDL.Func(
        [IDL.Text, ChainEnum, IDL.Text, IDL.Nat64],
        [SendResult],
        [],
      ),
    'account_swap_btc_to_ckbtc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Nat64],
        [BtcPending],
        [],
      ),
    'account_swap_ckbtc_to_btc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Text, IDL.Nat64],
        [IDL.Nat64],
        [],
      ),
    'account_top_up_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal)],
        [Result_1],
        [],
      ),
    'add_controller_and_update' : IDL.Func(
        [
          IDL.Principal,
          IDL.Text,
          IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
        ],
        [],
        [],
      ),
    'add_setting' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'canister_cycle_balance' : IDL.Func([], [IDL.Nat], ['query']),
    'canister_version' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account' : IDL.Func([IDL.Text], [WalletAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account_counters' : IDL.Func([], [AccountsNonce], ['query']),
    'get_account_view' : IDL.Func([IDL.Text], [WalletAccountView], ['query']),
    'get_account_views' : IDL.Func([], [IDL.Vec(WalletAccountView)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(ChainEnum, IDL.Text))],
        ['query'],
      ),
    'init_wallet' : IDL.Func([InititializeWalletArgs], [], []),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'refresh_settings' : IDL.Func([], [], []),
    'remove_setting' : IDL.Func([IDL.Text], [], []),
    'reset_accounts' : IDL.Func([], [], []),
    'retrieve_btc_status' : IDL.Func(
        [Minter, IDL.Nat64],
        [RetrieveBtcStatus],
        ['query'],
      ),
    'setting_and_signer' : IDL.Func([], [WalletSettings], ['query']),
    'status' : IDL.Func([], [WalletCanisterStatus], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'update_controller' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Principal, Controller))],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Controller))],
        [],
      ),
    'update_settings' : IDL.Func([], [], []),
    'upgrage_wallet' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_details' : IDL.Func([], [WasmDetails], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'wasm_hash_string' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
