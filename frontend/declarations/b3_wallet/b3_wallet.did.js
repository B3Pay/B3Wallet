export const idlFactory = ({ IDL }) => {
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
  const ChainType = IDL.Variant({
    'BTC' : BtcNetwork,
    'EVM' : IDL.Nat64,
    'ICP' : IDL.Null,
    'ICRC' : IDL.Principal,
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
  const UpdateBalanceError = IDL.Variant({
    'GenericError' : IDL.Record({
      'error_message' : IDL.Text,
      'error_code' : IDL.Nat64,
    }),
    'TemporarilyUnavailable' : IDL.Text,
    'AlreadyProcessing' : IDL.Null,
    'NoNewUtxos' : IDL.Record({
      'required_confirmations' : IDL.Nat32,
      'current_confirmations' : IDL.Opt(IDL.Nat32),
    }),
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Vec(UtxoStatus),
    'Err' : UpdateBalanceError,
  });
  const BTC = IDL.Record({ 'address' : IDL.Text, 'btc_network' : BtcNetwork });
  const EVM = IDL.Record({ 'chain_id' : IDL.Nat64, 'address' : IDL.Text });
  const Timestamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const ICP = IDL.Record({
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
  const ICRC = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, ICRC1MetadataValue)),
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'canister_id' : IDL.Principal,
    'subaccount' : IDL.Vec(IDL.Nat8),
    'created_at_time' : IDL.Opt(IDL.Nat64),
  });
  const Chain = IDL.Variant({
    'BTC' : BTC,
    'EVM' : EVM,
    'ICP' : ICP,
    'ICRC' : ICRC,
  });
  const Ledger = IDL.Record({
    'subaccount' : IDL.Vec(IDL.Nat8),
    'ecdsa' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'chains' : IDL.Vec(IDL.Tuple(ChainType, Chain)),
  });
  const WalletAccount = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'ledger' : Ledger,
  });
  const AccountsCounter = IDL.Record({
    'staging' : IDL.Nat64,
    'production' : IDL.Nat64,
    'development' : IDL.Nat64,
  });
  const WalletAccountView = IDL.Record({
    'id' : IDL.Text,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'name' : IDL.Text,
    'hidden' : IDL.Bool,
    'addresses' : IDL.Vec(IDL.Tuple(ChainType, IDL.Text)),
    'environment' : Environment,
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
  const BtcTransferRequest = IDL.Record({
    'to' : IDL.Text,
    'account_id' : IDL.Text,
    'network' : BtcNetwork,
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
  const RequestArgs = IDL.Record({
    'request' : Request,
    'role' : Roles,
    'deadline' : IDL.Opt(IDL.Nat64),
  });
  const ConsentPreferences = IDL.Record({ 'language' : IDL.Text });
  const ConsentMessageRequest = IDL.Record({
    'arg' : RequestArgs,
    'method' : IDL.Text,
    'consent_preferences' : ConsentPreferences,
  });
  const RequestResponse = IDL.Variant({
    'Reject' : IDL.Null,
    'Confirm' : IDL.Null,
  });
  const PendingRequest = IDL.Record({
    'id' : IDL.Nat64,
    'request' : Request,
    'role' : Roles,
    'deadline' : IDL.Nat64,
    'consent_message' : ConsentMessageRequest,
    'response' : IDL.Vec(IDL.Tuple(IDL.Principal, RequestResponse)),
  });
  const RequestStatus = IDL.Variant({
    'Fail' : IDL.Null,
    'Success' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const RequestError = IDL.Variant({
    'InvalidMessage' : IDL.Text,
    'InvalidMessageLength' : IDL.Null,
    'RequestAlreadySigned' : IDL.Text,
    'InvalidAddress' : IDL.Null,
    'CannotRemoveDefaultAccount' : IDL.Null,
    'RequestNotProcessed' : IDL.Nat64,
    'DeadlineExceeded' : IDL.Null,
    'InvalidController' : IDL.Null,
    'WalletAccountNotExists' : IDL.Null,
    'InvalidEvmTransactionType' : IDL.Null,
    'CyclesMintingError' : IDL.Text,
    'InvalidTx' : IDL.Text,
    'SignerRoleNotAuthorized' : IDL.Text,
    'RequestNotExists' : IDL.Null,
    'BitcoinGetBalanceError' : IDL.Text,
    'BitcoinInsufficientBalanceError' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'PublicKeyError' : IDL.Text,
    'RequestExpired' : IDL.Null,
    'NoUtxos' : IDL.Null,
    'UnknownError' : IDL.Null,
    'InvalidEcdsaPublicKey' : IDL.Null,
    'GenerateError' : IDL.Text,
    'InvalidTransaction' : IDL.Text,
    'InvalidSignature' : IDL.Text,
    'SignerRoleNotFound' : IDL.Tuple(IDL.Text, IDL.Text),
    'NotifyTopUpError' : IDL.Text,
    'MissingEcdsaPublicKey' : IDL.Null,
    'InvalidMsg' : IDL.Text,
    'SignerAlreadyExists' : IDL.Text,
    'BitcoinGetFeeRateError' : IDL.Text,
    'MissingSighashType' : IDL.Null,
    'WalletAccountAlreadyExists' : IDL.Null,
    'BitcoinGetUtxosError' : IDL.Text,
    'MissingAddress' : IDL.Null,
    'SignerDoesNotExist' : IDL.Text,
    'LedgerError' : IDL.Text,
    'RecoverableSignatureError' : IDL.Text,
    'InvalidAccountIdentifier' : IDL.Null,
    'RequestAlreadyProcessed' : IDL.Nat64,
    'InvalidPublicKey' : IDL.Text,
    'UpdateSettingsError' : IDL.Text,
    'SignError' : IDL.Text,
    'RequestNotFound' : IDL.Nat64,
    'BitcoinFeeTooHighError' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'WalletAccountCounterMismatch' : IDL.Null,
    'BitcoinGetAddressError' : IDL.Null,
    'InvalidRequest' : IDL.Null,
    'CallerIsNotOwner' : IDL.Null,
    'RequestRejected' : IDL.Null,
    'InvalidRecoveryId' : IDL.Text,
    'BitcoinInvalidFeePercentile' : IDL.Null,
    'InvalidNetwork' : IDL.Null,
    'BitcoinSignatureError' : IDL.Text,
    'InvalidNetworkAddress' : IDL.Null,
    'MissingWitnessScript' : IDL.Null,
    'SignerNotFound' : IDL.Text,
    'BitcoinGetCurrentFeePercentilesError' : IDL.Text,
    'Processing' : IDL.Null,
    'BitcoinSendTransactionError' : IDL.Text,
    'NotSignedTransaction' : IDL.Null,
    'ExecutionError' : IDL.Text,
    'TransactionTooOld' : IDL.Nat64,
    'CanisterStatusError' : IDL.Text,
    'EcdsaPublicKeyAlreadySet' : IDL.Null,
    'BitcoinSendRawTransactionError' : IDL.Text,
  });
  const ErrorInfo = IDL.Record({
    'description' : IDL.Text,
    'error_code' : IDL.Nat64,
  });
  const ConsentInfo = IDL.Record({
    'consent_message' : IDL.Text,
    'language' : IDL.Text,
  });
  const ConsentMessageResponse = IDL.Variant({
    'MalformedCall' : ErrorInfo,
    'Valid' : ConsentInfo,
    'Other' : IDL.Text,
    'Forbidden' : ErrorInfo,
  });
  const ProcessedRequest = IDL.Record({
    'status' : RequestStatus,
    'method' : IDL.Text,
    'request' : PendingRequest,
    'error' : IDL.Opt(RequestError),
    'message' : ConsentMessageResponse,
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
  const WalletCanisterStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
    'account_status' : AccountsCounter,
  });
  return IDL.Service({
    'account_balance_btc' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Opt(IDL.Nat32)],
        [IDL.Nat64],
        [],
      ),
    'account_btc_fees' : IDL.Func([BtcNetwork, IDL.Nat8], [IDL.Nat64], []),
    'account_btc_utxos' : IDL.Func(
        [IDL.Text, BtcNetwork, IDL.Opt(UtxoFilter)],
        [GetUtxosResponse],
        [],
      ),
    'account_create' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [],
        [],
      ),
    'account_generate_address' : IDL.Func([IDL.Text, ChainType], [], []),
    'account_hide' : IDL.Func([IDL.Text], [], []),
    'account_icp_balance' : IDL.Func([IDL.Text], [IDL.Nat], []),
    'account_icrc_balance' : IDL.Func([IDL.Text, IDL.Principal], [IDL.Nat], []),
    'account_remove' : IDL.Func([IDL.Text], [], []),
    'account_remove_address' : IDL.Func([IDL.Text, ChainType], [], []),
    'account_rename' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'account_restore' : IDL.Func([Environment, IDL.Nat64], [], []),
    'account_send' : IDL.Func(
        [IDL.Text, ChainType, IDL.Text, IDL.Nat64],
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
        [IDL.Text, IDL.Text, IDL.Nat64],
        [IDL.Text],
        [],
      ),
    'account_top_up_and_notify' : IDL.Func(
        [IDL.Text, Tokens, IDL.Opt(IDL.Principal), IDL.Opt(Tokens)],
        [IDL.Nat],
        [],
      ),
    'account_update_balance' : IDL.Func([IDL.Text], [Result], []),
    'canister_cycle_balance' : IDL.Func([], [IDL.Nat], ['query']),
    'canister_version' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account' : IDL.Func([IDL.Text], [WalletAccount], ['query']),
    'get_account_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_account_counters' : IDL.Func([], [AccountsCounter], ['query']),
    'get_account_view' : IDL.Func([IDL.Text], [WalletAccountView], ['query']),
    'get_account_views' : IDL.Func([], [IDL.Vec(WalletAccountView)], ['query']),
    'get_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(ChainType, IDL.Text))],
        ['query'],
      ),
    'get_balance' : IDL.Func([IDL.Nat64], [RetrieveBtcStatus], ['query']),
    'get_pending_list' : IDL.Func([], [IDL.Vec(PendingRequest)], ['query']),
    'get_processed' : IDL.Func([IDL.Nat64], [ProcessedRequest], ['query']),
    'get_processed_list' : IDL.Func([], [IDL.Vec(ProcessedRequest)], ['query']),
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
    'request_add_signer' : IDL.Func(
        [AddSignerRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_create_account' : IDL.Func(
        [CreateAccountRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_delete_account' : IDL.Func(
        [HideAccountRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_maker' : IDL.Func([Request, IDL.Opt(IDL.Nat64)], [IDL.Nat64], []),
    'request_response' : IDL.Func(
        [IDL.Nat64, RequestResponse],
        [ProcessedRequest],
        [],
      ),
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
    'request_transfer_btc' : IDL.Func(
        [BtcTransferRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
        [],
      ),
    'request_transfer_icp' : IDL.Func(
        [IcpTransferRequest, IDL.Opt(IDL.Nat64)],
        [IDL.Nat64],
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
    'status' : IDL.Func([], [WalletCanisterStatus], []),
    'unload_wasm' : IDL.Func([], [IDL.Nat64], []),
    'upgrage_wallet' : IDL.Func([], [], []),
    'validate_signer' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'version' : IDL.Func([], [IDL.Text], ['query']),
    'wasm_hash' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'wasm_hash_string' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
