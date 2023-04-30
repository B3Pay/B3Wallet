export const idlFactory = ({ IDL }) => {
  const UserControl = IDL.Record({
    'updated_at' : IDL.Nat64,
    'user_control_id' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Principal,
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : UserControl, 'Err' : IDL.Text });
  const Controller = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const LoadRelease = IDL.Record({ 'total' : IDL.Nat64, 'chunks' : IDL.Nat64 });
  return IDL.Service({
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'create_user_control' : IDL.Func([], [Result], []),
    'get_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Controller))],
        ['query'],
      ),
    'get_releases_version' : IDL.Func([], [IDL.Text], ['query']),
    'get_user_control' : IDL.Func([], [IDL.Opt(UserControl)], ['query']),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'load_release' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Text], [LoadRelease], []),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'reset_release' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
