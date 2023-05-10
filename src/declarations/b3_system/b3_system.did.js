export const idlFactory = ({ IDL }) => {
  const UserControl = IDL.Record({
    'updated_at' : IDL.Nat64,
    'user_control_id' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : UserControl, 'Err' : IDL.Text });
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
  const Result_1 = IDL.Variant({ 'Ok' : CanisterStatus, 'Err' : IDL.Text });
  const Release = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'date' : IDL.Nat64,
    'hash' : IDL.Text,
    'size' : IDL.Nat64,
    'version' : IDL.Text,
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
  const Result_2 = IDL.Variant({ 'Ok' : LoadRelease, 'Err' : IDL.Text });
  return IDL.Service({
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'create_user_control' : IDL.Func([], [Result], []),
    'get_canister_status' : IDL.Func([IDL.Principal], [Result_1], ['query']),
    'get_controllers' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_latest_release' : IDL.Func([], [Release], ['query']),
    'get_release' : IDL.Func([IDL.Nat64], [Release], ['query']),
    'get_releases' : IDL.Func([], [IDL.Vec(Release)], ['query']),
    'get_user_control' : IDL.Func([], [IDL.Opt(UserControl)], ['query']),
    'get_user_control_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Principal)],
        ['query'],
      ),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'load_release' : IDL.Func([IDL.Vec(IDL.Nat8), ReleaseArgs], [Result_2], []),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'remove_latest_release' : IDL.Func([], [], []),
    'remove_user_control' : IDL.Func([IDL.Principal], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
