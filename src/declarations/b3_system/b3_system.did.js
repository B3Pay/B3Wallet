export const idlFactory = ({ IDL }) => {
  const SignerCanister = IDL.Record({
    'updated_at' : IDL.Nat64,
    'canister_id' : IDL.Opt(IDL.Principal),
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : SignerCanister, 'Err' : IDL.Text });
  const SystemError = IDL.Variant({
    'UserAlreadyExists' : IDL.Null,
    'WasmGetError' : IDL.Text,
    'CreateCanisterError' : IDL.Text,
    'SignerCanisterAlreadyExists' : IDL.Text,
    'EncodeError' : IDL.Text,
    'SignerCanisterRateError' : IDL.Text,
    'InstallArgError' : IDL.Text,
    'SignerCanisterNotFound' : IDL.Null,
    'OwnerMismatch' : IDL.Record({ 'owner' : IDL.Text, 'user' : IDL.Text }),
    'InvalidAccountIdentifier' : IDL.Null,
    'UpdateControllersError' : IDL.Text,
    'ReleaseNotFound' : IDL.Null,
    'WasmNotFound' : IDL.Null,
    'WasmInstallError' : IDL.Text,
    'SignerCanisterAlreadyInstalled' : IDL.Null,
    'SignerCanisterDoesNotExist' : IDL.Text,
    'InstallCodeError' : IDL.Text,
    'UserNotFound' : IDL.Null,
    'CanisterStatusError' : IDL.Text,
    'WasmAlreadyLoaded' : IDL.Null,
    'ReleaseAlreadyExists' : IDL.Null,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : SystemError });
  const Release = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'date' : IDL.Nat64,
    'hash' : IDL.Vec(IDL.Nat8),
    'size' : IDL.Nat64,
    'version' : IDL.Text,
    'deprecated' : IDL.Bool,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Release, 'Err' : SystemError });
  const ReleaseArgs = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'size' : IDL.Nat64,
    'version' : IDL.Text,
  });
  const LoadRelease = IDL.Record({
    'total' : IDL.Nat64,
    'version' : IDL.Text,
    'chunks' : IDL.Nat64,
  });
  const Result_3 = IDL.Variant({ 'Ok' : LoadRelease, 'Err' : SystemError });
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
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'change_signer_canister' : IDL.Func([IDL.Principal], [], []),
    'create_signer_canister' : IDL.Func([], [Result], []),
    'deprecate_release' : IDL.Func([IDL.Text], [Result_1], []),
    'get_canister' : IDL.Func([], [SignerCanister], ['query']),
    'get_canister_version' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
    'get_canister_wasmhash' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Nat8)],
        ['query'],
      ),
    'get_controllers' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_release' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_release_by_index' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_signer_canisters' : IDL.Func([], [IDL.Vec(SignerCanister)], ['query']),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'install_signer_canister' : IDL.Func(
        [IDL.Opt(IDL.Principal)],
        [Result],
        [],
      ),
    'latest_release' : IDL.Func([], [Release], ['query']),
    'load_release' : IDL.Func([IDL.Vec(IDL.Nat8), ReleaseArgs], [Result_3], []),
    'releases' : IDL.Func([], [IDL.Vec(Release)], ['query']),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'remove_latest_release' : IDL.Func([], [], []),
    'remove_release' : IDL.Func([IDL.Text], [Result_2], []),
    'remove_signer_canister' : IDL.Func([IDL.Principal], [], []),
    'reset_users' : IDL.Func([], [], []),
    'status' : IDL.Func([], [CanisterStatus], []),
    'update_release' : IDL.Func([ReleaseArgs], [Result_1], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
