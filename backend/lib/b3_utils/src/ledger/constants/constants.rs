use crate::{ledger::currency::ICPToken, NanoTimeStamp, Subaccount};

pub const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0u8; 32]);

pub const SYSTEM_RATE_LIMIT: u64 = NanoTimeStamp::NS_PER_MINUTE;

pub const IC_TRANSACTION_FEE_ICP: ICPToken = ICPToken::from_e8s(10_000);
