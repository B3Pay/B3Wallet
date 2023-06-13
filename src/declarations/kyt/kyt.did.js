export const idlFactory = ({ IDL }) => {
  const Mode = IDL.Variant({
    'RejectAll' : IDL.Null,
    'Normal' : IDL.Null,
    'AcceptAll' : IDL.Null,
  });
  const UpgradeArg = IDL.Record({
    'maintainers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'mode' : IDL.Opt(Mode),
    'minter_id' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const InitArg = IDL.Record({
    'maintainers' : IDL.Vec(IDL.Principal),
    'mode' : Mode,
    'minter_id' : IDL.Principal,
  });
  const LifecycleArg = IDL.Variant({
    'UpgradeArg' : UpgradeArg,
    'InitArg' : InitArg,
  });
  const DepositRequest = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
    'caller' : IDL.Principal,
  });
  const Alert = IDL.Record({
    'service' : IDL.Opt(IDL.Text),
    'level' : IDL.Variant({
      'Low' : IDL.Null,
      'High' : IDL.Null,
      'Medium' : IDL.Null,
      'Severe' : IDL.Null,
    }),
    'category' : IDL.Opt(IDL.Text),
    'exposure_type' : IDL.Variant({
      'Indirect' : IDL.Null,
      'Direct' : IDL.Null,
    }),
  });
  const Response = IDL.Record({
    'provider' : IDL.Principal,
    'alerts' : IDL.Vec(Alert),
    'external_id' : IDL.Text,
  });
  const FetchUtxoAlertsError = IDL.Variant({
    'TemporarilyUnavailable' : IDL.Text,
  });
  const WithdrawalAttempt = IDL.Record({
    'id' : IDL.Text,
    'timestamp_nanos' : IDL.Nat64,
    'address' : IDL.Text,
    'caller' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  const FetchWithdrawalAlertsError = IDL.Variant({
    'TemporarilyUnavailable' : IDL.Text,
  });
  const SetApiKeyArg = IDL.Record({ 'api_key' : IDL.Text });
  return IDL.Service({
    'fetch_utxo_alerts' : IDL.Func(
        [DepositRequest],
        [IDL.Variant({ 'Ok' : Response, 'Err' : FetchUtxoAlertsError })],
        [],
      ),
    'fetch_withdrawal_alerts' : IDL.Func(
        [WithdrawalAttempt],
        [IDL.Variant({ 'Ok' : Response, 'Err' : FetchWithdrawalAlertsError })],
        [],
      ),
    'set_api_key' : IDL.Func([SetApiKeyArg], [], []),
    'txid_to_bytes' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Nat8)], ['query']),
  });
};
export const init = ({ IDL }) => {
  const Mode = IDL.Variant({
    'RejectAll' : IDL.Null,
    'Normal' : IDL.Null,
    'AcceptAll' : IDL.Null,
  });
  const UpgradeArg = IDL.Record({
    'maintainers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'mode' : IDL.Opt(Mode),
    'minter_id' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const InitArg = IDL.Record({
    'maintainers' : IDL.Vec(IDL.Principal),
    'mode' : Mode,
    'minter_id' : IDL.Principal,
  });
  const LifecycleArg = IDL.Variant({
    'UpgradeArg' : UpgradeArg,
    'InitArg' : InitArg,
  });
  return [LifecycleArg];
};
