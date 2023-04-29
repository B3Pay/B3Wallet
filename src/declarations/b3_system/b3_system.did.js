export const idlFactory = ({ IDL }) => {
  const Controller = IDL.Record({
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const LoadRelease = IDL.Record({ 'total' : IDL.Nat64, 'chunks' : IDL.Nat64 });
  return IDL.Service({
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'get_controllers' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, Controller))],
        ['query'],
      ),
    'get_releases_version' : IDL.Func([], [IDL.Text], ['query']),
    'load_release' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Text], [LoadRelease], []),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'reset_release' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
