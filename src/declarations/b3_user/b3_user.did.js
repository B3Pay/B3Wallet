export const idlFactory = ({ IDL }) => {
  const Environment = IDL.Variant({
    'Production' : IDL.Null,
    'Development' : IDL.Null,
    'Staging' : IDL.Null,
  });
  const PublicKey = IDL.Record({
    'address' : IDL.Text,
    'bytes' : IDL.Vec(IDL.Nat8),
  });
  const Result = IDL.Variant({ 'Ok' : PublicKey, 'Err' : IDL.Text });
  const Config = IDL.Record({
    'env' : Environment,
    'sign_cycles' : IDL.Nat64,
    'key_name' : IDL.Text,
  });
  const Derivation = IDL.Record({
    'path' : IDL.Vec(IDL.Nat8),
    'config' : Config,
  });
  const Status = IDL.Variant({
    'Failed' : IDL.Null,
    'Success' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const SignedTransaction = IDL.Record({
    'status' : Status,
    'data' : IDL.Vec(IDL.Nat8),
    'timestamp' : IDL.Nat64,
  });
  const ChainData = IDL.Record({
    'nonce' : IDL.Nat64,
    'transactions' : IDL.Vec(SignedTransaction),
  });
  const Account = IDL.Record({
    'derivation' : Derivation,
    'public_key' : PublicKey,
    'name' : IDL.Text,
    'chain_data' : IDL.Vec(IDL.Tuple(IDL.Nat64, ChainData)),
  });
  const Result_1 = IDL.Variant({ 'Ok' : SignedTransaction, 'Err' : IDL.Text });
  return IDL.Service({
    'create_account' : IDL.Func([Environment, IDL.Opt(IDL.Text)], [Result], []),
    'get_account' : IDL.Func([IDL.Nat8], [Account], ['query']),
    'get_accounts' : IDL.Func([], [IDL.Vec(Account)], ['query']),
    'get_public_key' : IDL.Func([IDL.Nat8], [PublicKey], ['query']),
    'sign_transaction' : IDL.Func(
        [IDL.Nat8, IDL.Nat64, IDL.Vec(IDL.Nat8)],
        [Result_1],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
