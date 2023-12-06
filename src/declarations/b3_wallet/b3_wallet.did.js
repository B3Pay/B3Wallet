export const idlFactory = ({ IDL }) => {
  const Value = IDL.Rec();
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
  const BtcPending = IDL.Record({ 'txid' : IDL.Text, 'account' : IDL.Text });
  const EvmPending = IDL.Record({ 'block_index' : IDL.Nat64 });
  const IcpPending = IDL.Record({
    'block_index' : IDL.Nat64,
    'canister_id' : IDL.Text,
  });
  const IcrcPending = IDL.Record({
    'tx_index' : IDL.Nat,
    'block_index' : IDL.Nat64,
  });
  const CkbtcPending = IDL.Record({
    'block_index' : IDL.Nat64,
    'txid' : IDL.Opt(IDL.Nat),
  });
  const PendingEnum = IDL.Variant({
    'BTC' : BtcPending,
    'EVM' : EvmPending,
    'ICP' : IcpPending,
    'ICRC' : IcrcPending,
    'CKBTC' : CkbtcPending,
  });
  const BtcNetwork = IDL.Variant({
    'Mainnet' : IDL.Null,
    'Regtest' : IDL.Null,
    'Testnet' : IDL.Null,
  });
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const TokenAmount = IDL.Record({ 'decimals' : IDL.Nat8, 'amount' : IDL.Nat });
  const SendResult = IDL.Variant({
    'BTC' : IDL.Text,
    'EVM' : IDL.Null,
    'ICP' : IDL.Nat64,
    'ICRC' : IDL.Nat,
    'CKBTC' : IDL.Nat,
  });
  const ICPToken = IDL.Record({ 'e8s' : IDL.Nat64 });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
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
  Value.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
      'Nat' : IDL.Nat,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Text' : IDL.Text,
      'Array' : IDL.Vec(Value),
    })
  );
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
  const ICPTransferTimestamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const IcpChain = IDL.Record({
    'fee' : ICPToken,
    'pendings' : IDL.Vec(IcpPending),
    'memo' : IDL.Nat64,
    'subaccount' : IDL.Vec(IDL.Nat8),
    'created_at_time' : IDL.Opt(ICPTransferTimestamp),
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
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'ledger' : Ledger,
  });
  const AppAccountsNonce = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const WalletAccountView = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
    'pendings' : IDL.Vec(PendingEnum),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'addresses' : IDL.Vec(IDL.Tuple(ChainEnum, IDL.Text)),
    'environment' : Environment,
  });
  const OperationStatus = IDL.Variant({
    'Fail' : IDL.Null,
    'Success' : IDL.Null,
    'Expired' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const Response = IDL.Variant({ 'Reject' : IDL.Null, 'Confirm' : IDL.Null });
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
  const IcpTransfer = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(ICPToken),
    'account_id' : IDL.Text,
    'memo' : IDL.Opt(IDL.Nat64),
    'amount' : ICPToken,
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
  const OperationEnum = IDL.Variant({
    'UnhideAccount' : IDL.Null,
    'EvmDeployContract' : IDL.Null,
    'IcpTransfer' : IDL.Null,
    'EvmSignRawTransaction' : IDL.Null,
    'EvmSignMessage' : IDL.Null,
    'UpdateCanisterSettings' : IDL.Null,
    'RenameAccount' : IDL.Null,
    'AddUser' : IDL.Null,
    'EvmSignTranscation' : IDL.Null,
    'EvmTransferErc20' : IDL.Null,
    'SendToken' : IDL.Null,
    'HideAccount' : IDL.Null,
    'UpgradeCanister' : IDL.Null,
    'TopUpTransfer' : IDL.Null,
    'BtcTransfer' : IDL.Null,
    'RemoveUser' : IDL.Null,
    'RemoveAccount' : IDL.Null,
    'CreateAccount' : IDL.Null,
    'EvmTransfer' : IDL.Null,
  });
  const OperationAccess = IDL.Record({
    'valid_until' : IDL.Opt(IDL.Nat64),
    'operation' : OperationEnum,
  });
  const AccessLevel = IDL.Variant({
    'ReadOnly' : IDL.Null,
    'Limited' : IDL.Vec(OperationAccess),
    'Canister' : IDL.Null,
    'FullAccess' : IDL.Null,
  });
  const Role = IDL.Record({ 'access_level' : AccessLevel, 'name' : IDL.Text });
  const AddUser = IDL.Record({
    'threshold' : IDL.Opt(IDL.Nat8),
    'name' : IDL.Text,
    'role' : Role,
    'signer_id' : IDL.Vec(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
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
  const SendToken = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'chain' : ChainEnum,
    'amount' : TokenAmount,
  });
  const UpgradeCanister = IDL.Record({
    'wasm_hash_string' : IDL.Text,
    'wasm_version' : IDL.Text,
  });
  const TopUpTransfer = IDL.Record({
    'fee' : IDL.Opt(ICPToken),
    'account_id' : IDL.Text,
    'canister_id' : IDL.Principal,
    'amount' : ICPToken,
  });
  const BtcTransfer = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'network' : Minter,
    'amount' : TokenAmount,
  });
  const RemoveUser = IDL.Record({ 'signer_id' : IDL.Vec(IDL.Nat8) });
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
  const Operation = IDL.Variant({
    'UnhideAccount' : HideAccount,
    'EvmDeployContract' : EvmDeployContract,
    'IcpTransfer' : IcpTransfer,
    'EvmSignRawTransaction' : EvmSignRawTransaction,
    'EvmSignMessage' : EvmSignMessage,
    'UpdateCanisterSettings' : UpdateCanisterSettings,
    'RenameAccount' : RenameAccount,
    'AddUser' : AddUser,
    'EvmSignTranscation' : EvmSignTranscation,
    'EvmTransferErc20' : EvmTransferErc20,
    'SendToken' : SendToken,
    'HideAccount' : HideAccount,
    'UpgradeCanister' : UpgradeCanister,
    'TopUpTransfer' : TopUpTransfer,
    'BtcTransfer' : BtcTransfer,
    'RemoveUser' : RemoveUser,
    'RemoveAccount' : HideAccount,
    'CreateAccount' : CreateAccount,
    'EvmTransfer' : EvmTransfer,
  });
  const ConsentMessage = IDL.Record({
    'title' : IDL.Text,
    'message' : IDL.Text,
    'reason' : IDL.Text,
  });
  const PendingOperation = IDL.Record({
    'id' : IDL.Nat64,
    'status' : OperationStatus,
    'responses' : IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), Response)),
    'allowed_signers' : IDL.Vec(IDL.Vec(IDL.Nat8)),
    'request' : Operation,
    'deadline' : IDL.Nat64,
    'consent_message' : ConsentMessage,
    'created_at' : IDL.Nat64,
    'created_by' : IDL.Vec(IDL.Nat8),
    'version' : IDL.Text,
  });
  const NotifyTopUp = IDL.Record({
    'account_id' : IDL.Text,
    'block_index' : IDL.Nat64,
    'canister_id' : IDL.Principal,
  });
  const EvmContractDeployed = IDL.Record({
    'transaction' : EvmTransaction1559,
    'contract_address' : IDL.Text,
  });
  const OperationResult = IDL.Variant({
    'Empty' : IDL.Null,
    'AccountCreated' : CreateAccount,
    'CanisterTopUped' : IDL.Tuple(NotifyTopUp, IDL.Nat),
    'BtcTransfered' : IDL.Tuple(BtcTransfer, IDL.Text),
    'IcpTransfered' : IDL.Tuple(IcpTransfer, IDL.Nat64),
    'TokenSent' : IDL.Tuple(SendToken, SendResult),
    'AccountRenamed' : RenameAccount,
    'EvmContractDeployed' : EvmContractDeployed,
    'EvmErc20Transfered' : IDL.Tuple(EvmTransferErc20, IDL.Text),
    'SignerRemoved' : RemoveUser,
    'EvmTransfered' : IDL.Tuple(EvmTransfer, IDL.Text),
    'EvmRawTransactionSigned' : IDL.Tuple(EvmSignRawTransaction, IDL.Text),
    'TopUpTransfered' : IDL.Tuple(TopUpTransfer, IDL.Nat64),
    'AccountHidden' : HideAccount,
    'EvmMessageSigned' : IDL.Tuple(EvmSignMessage, IDL.Vec(IDL.Nat8)),
    'CanisterSettingsUpdated' : UpdateCanisterSettings,
    'SignerAdded' : AddUser,
    'CanisterUpgraded' : UpgradeCanister,
    'EvmTransactionSigned' : IDL.Tuple(EvmSignTranscation, IDL.Text),
    'AccountUnhidden' : HideAccount,
    'AccountRemoved' : HideAccount,
  });
  const ProcessedOperation = IDL.Record({
    'status' : OperationStatus,
    'result' : OperationResult,
    'method' : IDL.Text,
    'error' : IDL.Opt(IDL.Text),
    'operation' : PendingOperation,
    'timestamp' : IDL.Nat64,
  });
  const User = IDL.Record({
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
    'name' : IDL.Text,
    'role' : Role,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const LogVariant = IDL.Variant({
    'info' : IDL.Null,
    'warn' : IDL.Null,
    'error' : IDL.Null,
  });
  const LogEntry = IDL.Record({
    'counter' : IDL.Nat64,
    'file' : IDL.Text,
    'line' : IDL.Nat32,
    'cycle' : IDL.Opt(IDL.Nat),
    'version' : IDL.Text,
    'message' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'variant' : LogVariant,
  });
  const Result_1 = IDL.Variant({ 'Ok' : ProcessedOperation, 'Err' : IDL.Text });
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
    'controllers' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
    'initialised' : IDL.Bool,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const WalletSettingsAndSigners = IDL.Record({
    'signers' : IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), User)),
    'settings' : WalletSettings,
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
  const QueryStats = IDL.Record({
    'response_payload_bytes_total' : IDL.Nat,
    'num_instructions_total' : IDL.Nat,
    'num_calls_total' : IDL.Nat,
    'request_payload_bytes_total' : IDL.Nat,
  });
  const CanisterStatusResponse = IDL.Record({
    'status' : CanisterStatusType,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'query_stats' : QueryStats,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const AppStatus = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AppAccountsNonce,
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
        [],
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
        [IDL.Text, ChainEnum, IDL.Text, TokenAmount],
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
        [IDL.Text, ICPToken, IDL.Opt(IDL.Principal)],
        [Result],
        [],
      ),
    'account_update_balance' : IDL.Func(
        [IDL.Text, BtcNetwork],
        [IDL.Vec(UtxoStatus)],
        [],
      ),
    'add_controller_and_update' : IDL.Func([IDL.Principal, IDL.Text], [], []),
    'add_setting_metadata' : IDL.Func([IDL.Text, Value], [], []),
    'canister_cycle_balance' : IDL.Func([], [IDL.Nat], ['query']),
    'canister_version' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account' : IDL.Func([IDL.Text], [WalletAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account_counters' : IDL.Func([], [AppAccountsNonce], ['query']),
    'get_account_view' : IDL.Func([IDL.Text], [WalletAccountView], ['query']),
    'get_account_views' : IDL.Func([], [IDL.Vec(WalletAccountView)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(ChainEnum, IDL.Text))],
        ['query'],
      ),
    'get_pending_list' : IDL.Func([], [IDL.Vec(PendingOperation)], ['query']),
    'get_processed_list' : IDL.Func(
        [],
        [IDL.Vec(ProcessedOperation)],
        ['query'],
      ),
    'get_roles' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, Role))],
        ['query'],
      ),
    'get_signers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), User))],
        ['query'],
      ),
    'init_wallet' : IDL.Func(
        [
          IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
          IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, Value))),
        ],
        [],
        [],
      ),
    'is_connected' : IDL.Func([], [IDL.Bool], ['query']),
    'load_wasm' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Nat64], []),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'print_log_entries' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
    'refresh_settings' : IDL.Func([], [], []),
    'remove_setting_metadata' : IDL.Func([IDL.Text], [], []),
    'report_bug' : IDL.Func([IDL.Principal, IDL.Text], [], []),
    'request_account_rename' : IDL.Func(
        [RenameAccount, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_add_signer' : IDL.Func(
        [AddUser, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_connect' : IDL.Func([IDL.Text], [IDL.Nat64], []),
    'request_create_account' : IDL.Func(
        [CreateAccount, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_delete_account' : IDL.Func(
        [HideAccount, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_maker' : IDL.Func(
        [Operation, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_remove_signer' : IDL.Func(
        [RemoveUser, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_send' : IDL.Func(
        [SendToken, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_transfer_btc' : IDL.Func(
        [BtcTransfer, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_transfer_icp' : IDL.Func(
        [IcpTransfer, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_update_settings' : IDL.Func(
        [UpdateCanisterSettings, IDL.Text, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_upgrade_canister' : IDL.Func([IDL.Text], [IDL.Nat64], []),
    'reset_accounts' : IDL.Func([], [], []),
    'response' : IDL.Func([IDL.Nat64, Response], [Result_1], []),
    'retrieve_btc_status' : IDL.Func(
        [Minter, IDL.Nat64],
        [RetrieveBtcStatus],
        ['query'],
      ),
    'role_add' : IDL.Func([Role], [IDL.Vec(IDL.Tuple(IDL.Nat64, Role))], []),
    'role_remove' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Tuple(IDL.Nat64, Role))],
        [],
      ),
    'setting_and_signer' : IDL.Func([], [WalletSettingsAndSigners], ['query']),
    'signer_add' : IDL.Func(
        [IDL.Vec(IDL.Nat8), Role],
        [IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), User))],
        [],
      ),
    'signer_remove' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), User))],
        [],
      ),
    'status' : IDL.Func([], [AppStatus], []),
    'uninstall_wallet' : IDL.Func([], [], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'update_controller' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))],
        [],
      ),
    'update_settings' : IDL.Func([], [], []),
    'upgrage_wallet' : IDL.Func([], [], []),
    'validate_signer' : IDL.Func([IDL.Vec(IDL.Nat8)], [IDL.Bool], ['query']),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_details' : IDL.Func([], [WasmDetails], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'wasm_hash_string' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
