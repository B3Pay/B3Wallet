export const idlFactory = ({ IDL }) => {
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const Keys = IDL.Record({
    'address' : IDL.Text,
    'bytes' : IDL.Vec(IDL.Nat8),
  });
  const Ecdsa = IDL.Record({ 'env' : Environment, 'path' : IDL.Vec(IDL.Nat8) });
  const Allowance = IDL.Record({
    'updated_at' : IDL.Nat64,
    'metadata' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'created_at' : IDL.Nat64,
    'limit' : IDL.Opt(IDL.Nat8),
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const SignRequest = IDL.Record({
    'id' : IDL.Nat64,
    'destination' : IDL.Principal,
    'public_key' : Keys,
    'data' : IDL.Vec(IDL.Nat8),
    'deadline' : IDL.Nat64,
    'cycles' : IDL.Nat64,
    'chain_id' : IDL.Nat64,
    'nonce' : IDL.Nat64,
  });
  const SignedTransaction = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const Account = IDL.Record({
    'id' : IDL.Text,
    'keys' : Keys,
    'name' : IDL.Text,
    'ecdsa' : Ecdsa,
    'canisters' : IDL.Vec(IDL.Tuple(IDL.Principal, Allowance)),
    'requests' : IDL.Vec(SignRequest),
    'signed' : SignedTransaction,
  });
  const Result = IDL.Variant({ 'Ok' : Account, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : SignedTransaction, 'Err' : IDL.Text });
  return IDL.Service({
    'change_owner' : IDL.Func([IDL.Principal], [], []),
    'create_account' : IDL.Func(
        [IDL.Opt(Environment), IDL.Opt(IDL.Text)],
        [Result],
        [],
      ),
    'get_account' : IDL.Func([IDL.Text], [Account], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(Account)], ['query']),
    'get_caller' : IDL.Func([], [IDL.Principal], ['query']),
    'get_owner' : IDL.Func([], [IDL.Principal], ['query']),
    'get_public_key' : IDL.Func([IDL.Text], [Keys], ['query']),
    'get_signature' : IDL.Func([IDL.Text, IDL.Vec(IDL.Nat8)], [Result_1], []),
    'get_signed' : IDL.Func([IDL.Text], [SignedTransaction], ['query']),
    'number_of_accounts' : IDL.Func([], [IDL.Nat8], ['query']),
    'sign_transaction' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Vec(IDL.Nat8)],
        [Result_2],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
