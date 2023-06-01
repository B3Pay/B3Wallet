export const idlFactory = ({ IDL }) => {
  const WalletCanister = IDL.Record({
    'updated_at' : IDL.Nat64,
    'canister_id' : IDL.Opt(IDL.Principal),
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : WalletCanister, 'Err' : IDL.Text });
  const Release = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'date' : IDL.Nat64,
    'hash' : IDL.Vec(IDL.Nat8),
    'size' : IDL.Nat64,
    'version' : IDL.Text,
    'deprecated' : IDL.Bool,
  });
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
  const SystemCanisterStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'user_status' : IDL.Nat64,
    'status_at' : IDL.Nat64,
    'version' : IDL.Text,
    'canister_status' : CanisterStatusResponse,
  });
  return IDL.Service({
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'add_wallet_canister' : IDL.Func([IDL.Principal], [], []),
    'change_wallet_canister' : IDL.Func([IDL.Principal], [], []),
    'create_wallet_canister' : IDL.Func([], [Result], []),
    'deprecate_release' : IDL.Func([IDL.Text], [], []),
    'get_canister' : IDL.Func([], [WalletCanister], ['query']),
    'get_canister_version' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
    'get_canister_version_by_user' : IDL.Func(
        [IDL.Principal],
        [IDL.Text],
        ['query'],
      ),
    'get_controllers' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_release' : IDL.Func([IDL.Text], [Release], ['query']),
    'get_release_by_index' : IDL.Func([IDL.Nat64], [Release], ['query']),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_wallet_canisters' : IDL.Func([], [IDL.Vec(WalletCanister)], ['query']),
    'get_wallet_release' : IDL.Func([IDL.Principal], [Release], ['query']),
    'install_wallet_canister' : IDL.Func(
        [IDL.Opt(IDL.Principal)],
        [Result],
        [],
      ),
    'latest_release' : IDL.Func([], [Release], ['query']),
    'load_release' : IDL.Func(
        [IDL.Vec(IDL.Nat8), ReleaseArgs],
        [LoadRelease],
        [],
      ),
    'releases' : IDL.Func([], [IDL.Vec(Release)], ['query']),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'remove_latest_release' : IDL.Func([], [], []),
    'remove_release' : IDL.Func([IDL.Text], [Release], []),
    'remove_wallet_canister' : IDL.Func([IDL.Principal], [], []),
    'reset_users' : IDL.Func([], [], []),
    'status' : IDL.Func([], [SystemCanisterStatus], []),
    'update_release' : IDL.Func([ReleaseArgs], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
