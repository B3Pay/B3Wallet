use crate::types::CanisterId;

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

// VETKD ----------------------------------------------------------------------

// s55qq-oqaaa-aaaaa-aaakq-cai
const VETKD_ID_TESTNET: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 21, 1, 1];

pub const VETKD_CANISTER_TESTNET: CanisterId = CanisterId::from_slice(&VETKD_ID_TESTNET);
