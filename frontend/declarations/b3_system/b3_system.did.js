export const idlFactory = ({ IDL }) => {
  const UserState = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'canisters' : IDL.Vec(IDL.Principal),
  });
  const Result = IDL.Variant({ 'Ok' : UserState, 'Err' : IDL.Text });
  const Bug = IDL.Record({
    'hash' : IDL.Vec(IDL.Nat8),
    'logs' : IDL.Vec(IDL.Text),
    'name' : IDL.Text,
    'canister_id' : IDL.Principal,
    'description' : IDL.Text,
    'version' : IDL.Text,
  });
  const Release = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'date' : IDL.Nat64,
    'hash' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
    'size' : IDL.Nat64,
    'version' : IDL.Text,
    'deprecated' : IDL.Bool,
  });
  const ReleaseArgs = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'name' : IDL.Text,
    'size' : IDL.Nat64,
    'version' : IDL.Text,
  });
  const LoadRelease = IDL.Record({
    'total' : IDL.Nat64,
    'version' : IDL.Text,
    'chunks' : IDL.Nat64,
  });
  const ReleaseNames = IDL.Variant({
    'b3_wallet' : IDL.Null,
    'Custom' : IDL.Text,
    'b3_multi_sig_wallet' : IDL.Null,
    'b3_basic_wallet' : IDL.Null,
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
    'change_wallet_canister' : IDL.Func([IDL.Principal, IDL.Nat64], [], []),
    'clear_bugs' : IDL.Func([], [], []),
    'create_wallet_canister' : IDL.Func([IDL.Text], [Result], []),
    'deprecate_release' : IDL.Func([IDL.Text, IDL.Text], [], []),
    'get_bugs' : IDL.Func([], [IDL.Vec(Bug)], ['query']),
    'get_canister_version' : IDL.Func([IDL.Principal], [IDL.Text], []),
    'get_canister_version_by_user' : IDL.Func(
        [IDL.Principal, IDL.Nat64],
        [IDL.Text],
        [],
      ),
    'get_canisters' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_controllers' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_create_canister_wallet_cycle' : IDL.Func([], [IDL.Nat], ['query']),
    'get_release' : IDL.Func([IDL.Text, IDL.Text], [Release], ['query']),
    'get_release_by_hash_string' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [Release],
        ['query'],
      ),
    'get_release_by_index' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [Release],
        ['query'],
      ),
    'get_states' : IDL.Func([], [UserState], ['query']),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_user_states' : IDL.Func([], [IDL.Vec(UserState)], ['query']),
    'install_wallet_canister' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result],
        [],
      ),
    'latest_release' : IDL.Func([IDL.Text], [Release], ['query']),
    'load_release' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), ReleaseArgs],
        [LoadRelease],
        [],
      ),
    'release_map' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(ReleaseNames, IDL.Vec(Release)))],
        ['query'],
      ),
    'releases' : IDL.Func([IDL.Text], [IDL.Vec(Release)], ['query']),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'remove_latest_release' : IDL.Func([IDL.Text], [], []),
    'remove_release' : IDL.Func([IDL.Text, IDL.Text], [Release], []),
    'remove_wallet_canister' : IDL.Func([IDL.Principal], [], []),
    'report_bug' : IDL.Func([Bug], [], []),
    'reset_users' : IDL.Func([], [], []),
    'status' : IDL.Func([], [SystemCanisterStatus], []),
    'update_release' : IDL.Func([IDL.Text, ReleaseArgs], [], []),
    'version' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
