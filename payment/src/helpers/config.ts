export const B3_PAYMENT_CANISTER_ID =
  process.env.B3_PAYMENT_CANISTER_ID ?? "b77ix-eeaaa-aaaaa-qaada-cai"

export const B3_SYSTEM_CANISTER_ID =
  process.env.B3_SYSTEM_CANISTER_ID ?? "rrkah-fqaaa-aaaaa-aaaaq-cai"

export const B3_USER_CANISTER_ID =
  process.env.B3_USER_CANISTER_ID ?? "rno2w-sqaaa-aaaaa-aaacq-cai"

export const IDENTITY_CANISTER_ID =
  process.env.INTERNET_IDENTITY_CANISTER_ID ?? "qhbym-qaaaa-aaaaa-aaafq-cai"

export const CMC_CANISTER_ID =
  process.env.CMC_CANISTER_ID ?? "rkp4c-7iaaa-aaaaa-aaaca-cai"

export const LEDGER_CANISTER_ID =
  process.env.LEDGER_CANISTER_ID ?? "ryjl3-tyaaa-aaaaa-aaaba-cai"

export const IS_LOCAL = process.env.DFX_NETWORK !== "ic"
