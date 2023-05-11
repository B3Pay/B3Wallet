export const idlFactory = ({ IDL }) => {
  const Signer = IDL.Record({
    'updated_at' : IDL.Nat64,
    'owner' : IDL.Principal,
    'signer_id' : IDL.Opt(IDL.Principal),
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : Signer, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Release = IDL.Record({
    'features' : IDL.Opt(IDL.Vec(IDL.Text)),
    'date' : IDL.Nat64,
    'hash' : IDL.Text,
    'size' : IDL.Nat64,
    'version' : IDL.Text,
    'deprecated' : IDL.Bool,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Release, 'Err' : IDL.Text });
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
  const Result_3 = IDL.Variant({ 'Ok' : LoadRelease, 'Err' : IDL.Text });
  return IDL.Service({
    'add_controller' : IDL.Func([IDL.Principal], [], []),
    'create_signer' : IDL.Func([], [Result], []),
    'deprecate_release' : IDL.Func([IDL.Text], [Result_1], []),
    'get_release' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_release_by_index' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_signer' : IDL.Func([], [IDL.Opt(Signer)], ['query']),
    'get_signer_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Principal)],
        ['query'],
      ),
    'get_signers' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_user_ids' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'latest_release' : IDL.Func([], [Release], ['query']),
    'load_release' : IDL.Func([IDL.Vec(IDL.Nat8), ReleaseArgs], [Result_3], []),
    'releases' : IDL.Func([], [IDL.Vec(Release)], ['query']),
    'remove_controller' : IDL.Func([IDL.Principal], [], []),
    'remove_latest_release' : IDL.Func([], [], []),
    'remove_release' : IDL.Func([IDL.Text], [Result_1], []),
    'remove_signer' : IDL.Func([IDL.Principal], [], []),
    'update_release' : IDL.Func([ReleaseArgs], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
