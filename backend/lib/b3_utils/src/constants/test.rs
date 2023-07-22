#[cfg(test)]
mod tests {
    use crate::{constants::*, types::CanisterId};

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
