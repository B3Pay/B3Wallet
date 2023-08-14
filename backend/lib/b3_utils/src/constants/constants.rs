use crate::{currency::ICPToken, NanoTimeStamp, Subaccount};

pub const DEVELOPMENT_PREFIX_NUMBER: u8 = 255;

pub const STAGING_PREFIX_NUMBER: u8 = 170;

pub const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0u8; 32]);

pub const SYSTEM_RATE_LIMIT: u64 = NanoTimeStamp::NS_PER_MINUTE;

pub const IC_TRANSACTION_FEE_ICP: ICPToken = ICPToken::from_e8s(10_000);
