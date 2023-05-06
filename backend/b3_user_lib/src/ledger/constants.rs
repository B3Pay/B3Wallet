use crate::types::Memo;

use super::ledger::Tokens;

pub const IC_TRANSACTION_FEE_ICP: Tokens = Tokens::from_e8s(10_000);

pub const CREATE_SIGNER_CYCLES: u128 = 1_000_000_000_000;

pub const CANISTER_CREATE_MEMO: Memo = Memo(0x41455243);

pub const CANISTER_TOP_UP_MEMO: Memo = Memo(0x50555054);

pub const TRANSFER_MEMO: Memo = Memo(0x544153);
