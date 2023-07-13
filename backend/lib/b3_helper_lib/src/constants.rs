use crate::{
    subaccount::Subaccount,
    time::NanoTimeStamp,
    tokens::Tokens,
    types::{CanisterId, Memo},
};

pub const DEVELOPMENT_PREFIX: u8 = 255;

pub const STAGING_PREFIX: u8 = 170;

pub const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0u8; 32]);

pub const RATE_LIMIT: u64 = NanoTimeStamp::NS_PER_MINUTE;

pub const IC_TRANSACTION_FEE_ICP: Tokens = Tokens::from_e8s(10_000);

pub const CREATE_WALLET_CANISTER_CYCLES: u128 = 200_000_000_000;

pub const CANISTER_CREATE_MEMO: Memo = Memo(0x41455243);

pub const CANISTER_TOP_UP_MEMO: Memo = Memo(0x50555054);

pub const CANISTER_TRANSFER_MEMO: Memo = Memo(0x544153);

pub const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;

pub const GET_UTXOS_COST_CYCLES: u64 = 10_000_000_000;

pub const GET_CURRENT_FEE_PERCENTILES_CYCLES: u64 = 100_000_000;

pub const SEND_TRANSACTION_BASE_CYCLES: u64 = 5_000_000_000;

pub const SEND_TRANSACTION_PER_BYTE_CYCLES: u64 = 20_000_000;

// aaaaa-aa
pub const MANAGMENT_CANISTER_ID: CanisterId = CanisterId::management_canister();

// ryjl3-tyaaa-aaaaa-aaaba-cai
const LEDGER_ID: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 2, 1, 1];

pub const LEDGER_CANISTER_ID: CanisterId = CanisterId::from_slice(&LEDGER_ID);

// rkp4c-7iaaa-aaaaa-aaaca-cai
const CMC_ID: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 4, 1, 1];

pub const CYCLES_MINTING_CANISTER_ID: CanisterId = CanisterId::from_slice(&CMC_ID);

// mxzaz-hqaaa-aaaar-qaada-cai
const CKBTC_LEDGER_ID_MAINNET: [u8; 10] = [0, 0, 0, 0, 2, 48, 0, 6, 1, 1];

pub const CKBTC_LEDGER_CANISTER_MAINNET: CanisterId =
    CanisterId::from_slice(&CKBTC_LEDGER_ID_MAINNET);

// mqygn-kiaaa-aaaar-qaadq-cai
const CKBTC_MINTER_ID_MAINNET: [u8; 10] = [0, 0, 0, 0, 2, 48, 0, 7, 1, 1];

pub const CKBTC_MINTER_CANISTER_MAINNET: CanisterId =
    CanisterId::from_slice(&CKBTC_MINTER_ID_MAINNET);

// TESTNET ---------------------------------------------------------------------

// mc6ru-gyaaa-aaaar-qaaaq-cai
const CKBTC_LEDGER_ID_TESTNET: [u8; 10] = [0, 0, 0, 0, 2, 48, 0, 1, 1, 1];

pub const CKBTC_LEDGER_CANISTER_TESTNET: CanisterId =
    CanisterId::from_slice(&CKBTC_LEDGER_ID_TESTNET);

// ml52i-qqaaa-aaaar-qaaba-cai
const CKBTC_MINTER_ID_TESTNET: [u8; 10] = [0, 0, 0, 0, 2, 48, 0, 2, 1, 1];

pub const CKBTC_MINTER_CANISTER_TESTNET: CanisterId =
    CanisterId::from_slice(&CKBTC_MINTER_ID_TESTNET);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger() {
        let ledger = CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        assert_eq!(ledger, LEDGER_CANISTER_ID);
    }

    #[test]
    fn test_management() {
        let management = CanisterId::from_text("aaaaa-aa").unwrap();

        assert_eq!(management, MANAGMENT_CANISTER_ID);
    }

    #[test]
    fn test_cmc() {
        let cmc = CanisterId::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap();

        assert_eq!(cmc, CYCLES_MINTING_CANISTER_ID);
    }

    #[test]
    fn test_ledger_ckbtc() {
        let ledger = CanisterId::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();

        assert_eq!(ledger, CKBTC_LEDGER_CANISTER_MAINNET);
    }

    #[test]
    fn test_minter() {
        let minter = CanisterId::from_text("mqygn-kiaaa-aaaar-qaadq-cai").unwrap();

        assert_eq!(minter, CKBTC_MINTER_CANISTER_MAINNET);
    }

    #[test]
    fn test_testnet_ledger() {
        let minter = CanisterId::from_text("mc6ru-gyaaa-aaaar-qaaaq-cai").unwrap();

        assert_eq!(minter, CKBTC_LEDGER_CANISTER_TESTNET);
    }

    #[test]
    fn test_testnet_minter() {
        let minter = CanisterId::from_text("ml52i-qqaaa-aaaar-qaaba-cai").unwrap();

        assert_eq!(minter, CKBTC_MINTER_CANISTER_TESTNET);
    }
}
