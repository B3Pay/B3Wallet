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
  const BtcNetwork = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const UtxoFilter = IDL.Variant({
    'page' : IDL.Vec(IDL.Nat8),
    'min_confirmations' : IDL.Nat32,
  });
  const Outpoint = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
  });
  const Utxo = IDL.Record({
    'height' : IDL.Nat32,
    'value' : IDL.Nat64,
    'outpoint' : Outpoint,
  });
  const GetUtxosResponse = IDL.Record({
    'next_page' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'tip_height' : IDL.Nat32,
    'tip_block_hash' : IDL.Vec(IDL.Nat8),
    'utxos' : IDL.Vec(Utxo),
  });
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const OutPoint = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
  });
  const Utxo_1 = IDL.Record({
    'height' : IDL.Nat32,
    'value' : IDL.Nat64,
    'outpoint' : OutPoint,
  });
  const UtxoStatus = IDL.Variant({
    'ValueTooSmall' : Utxo_1,
    'Tainted' : Utxo_1,
    'Minted' : IDL.Record({
      'minted_amount' : IDL.Nat64,
      'block_index' : IDL.Nat64,
      'utxo' : Utxo_1,
    }),
    'Checked' : Utxo_1,
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
  const EvmChain = IDL.Record({ 'chain_id' : IDL.Nat64, 'address' : IDL.Text });
  const BtcChain = IDL.Record({ 'address' : IDL.Text, 'btc_network' : Minter });
  const Timestamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const IcpChain = IDL.Record({
    'fee' : Tokens,
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
    'pending_sends' : IDL.Vec(
      IDL.Tuple(Minter, IDL.Vec(IDL.Tuple(IDL.Nat64, RetrieveBtcStatus)))
    ),
    'subaccount' : IDL.Vec(IDL.Nat8),
    'pending_receives' : IDL.Vec(IDL.Tuple(Minter, IDL.Text)),
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
    'pending_send' : IDL.Vec(
      IDL.Tuple(Minter, IDL.Vec(IDL.Tuple(IDL.Nat64, RetrieveBtcStatus)))
    ),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'pending_receive' : IDL.Vec(IDL.Tuple(Minter, IDL.Text)),
    'addresses' : IDL.Vec(IDL.Tuple(ChainEnum, IDL.Text)),
    'environment' : Environment,
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
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AccountsNonce,
  });
  return IDL.Service({
    'account_balance' : IDL.Func([IDL.Text, ChainEnum], [IDL.Nat], []),
    'account_balance_btc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Opt(IDL.Nat32)],
        [IDL.Nat],
        [],
      ),
    'account_btc_fees' : IDL.Func([BtcNetwork, IDL.Nat8], [IDL.Nat64], []),
    'account_btc_utxos' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Opt(UtxoFilter)],
        [GetUtxosResponse],
        [],
      ),
    'account_ckbtc_balance' : IDL.Func([IDL.Text, BtcNetwork], [IDL.Nat], []),
    'account_create' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [],
        [],
      ),
    'account_create_address' : IDL.Func([IDL.Text, ChainEnum], [], []),
    'account_hide' : IDL.Func([IDL.Text], [], []),
    'account_icp_balance' : IDL.Func([IDL.Text], [IDL.Nat], []),
    'account_icrc_balance' : IDL.Func([IDL.Text, IDL.Principal], [IDL.Nat], []),
    'account_remove' : IDL.Func([IDL.Text], [], []),
    'account_remove_address' : IDL.Func([IDL.Text, ChainEnum], [], []),
    'account_remove_pending_receive' : IDL.Func([IDL.Text, BtcNetwork], [], []),
    'account_remove_pending_send' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Nat64],
        [],
        [],
      ),
    'account_rename' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'account_restore' : IDL.Func([Environment, IDL.Nat64], [], []),
    'account_send' : IDL.Func(
        [IDL.Text, ChainEnum, IDL.Text, IDL.Nat64],
        [],
        [],
      ),
    'account_send_btc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Text, IDL.Nat64],
        [IDL.Text],
        [],
      ),
    'account_send_icp' : IDL.Func(
        [IDL.Text, IDL.Text, Tokens, IDL.Opt(Tokens), IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'account_swap_btc_to_ckbtc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Nat64],
        [IDL.Text],
        [],
      ),
    'account_swap_ckbtc_to_btc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Text, IDL.Nat64],
        [IDL.Nat64],
        [],
      ),
    'account_top_up_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal), IDL.Opt(Tokens)],
        [IDL.Nat],
        [],
      ),
    'account_update_receive_pending' : IDL.Func(
        [IDL.Text, BtcNetwork],
        [IDL.Vec(UtxoStatus)],
        [],
      ),
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
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'reset_wallet' : IDL.Func([], [], []),
    'retrieve_btc_status' : IDL.Func(
        [Minter, IDL.Nat64],
        [RetrieveBtcStatus],
        ['query'],
      ),
    'status' : IDL.Func([], [WalletCanisterStatus], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'upgrage_wallet' : IDL.Func([], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'wasm_hash_string' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
