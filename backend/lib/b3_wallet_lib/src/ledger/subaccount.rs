use super::error::LedgerError;
use super::types::{
    ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAResponse,
};
use super::{config::EcdsaConfig, types::EcdsaKeyId};
use async_trait::async_trait;
use b3_utils::constants::MANAGMENT_CANISTER_ID;
use b3_utils::Subaccount;
use ic_cdk::api::call::{call, call_with_payment};

#[async_trait]
pub trait SubaccountEcdsaTrait {
    fn config(&self) -> EcdsaConfig;
    fn key_id(&self) -> EcdsaKeyId;
    fn key_id_with_cycles_and_path(&self) -> (EcdsaKeyId, u64, Vec<Vec<u8>>);
    async fn ecdsa_public_key(&self) -> Result<Vec<u8>, LedgerError>;
    async fn sign_with_ecdsa(&self, message_hash: Vec<u8>) -> Result<Vec<u8>, LedgerError>;
}

#[async_trait]
impl SubaccountEcdsaTrait for Subaccount {
    fn config(&self) -> EcdsaConfig {
        self.environment().into()
    }

    fn key_id(&self) -> EcdsaKeyId {
        self.config().key_id()
    }

    fn key_id_with_cycles_and_path(&self) -> (EcdsaKeyId, u64, Vec<Vec<u8>>) {
        let config = self.config();

        (
            config.key_id(),
            config.sign_cycles(),
            self.derivation_path(),
        )
    }

    async fn ecdsa_public_key(&self) -> Result<Vec<u8>, LedgerError> {
        let key_id = self.key_id();

        let derivation_path = self.derivation_path();

        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path,
            key_id,
        };

        let (res,): (ECDSAPublicKeyResponse,) =
            call(MANAGMENT_CANISTER_ID, "ecdsa_public_key", (request,))
                .await
                .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(res.public_key)
    }

    async fn sign_with_ecdsa(&self, message_hash: Vec<u8>) -> Result<Vec<u8>, LedgerError> {
        let (key_id, cycles, derivation_path) = self.key_id_with_cycles_and_path();

        let request = SignWithECDSAArgs {
            derivation_path,
            message_hash,
            key_id,
        };

        let (res,): (SignWithECDSAResponse,) =
            call_with_payment(MANAGMENT_CANISTER_ID, "sign_with_ecdsa", (request,), cycles)
                .await
                .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(res.signature)
    }
}

#[cfg(test)]
mod tests {
    use b3_utils::{ledger::AccountIdentifier, Environment};
    use candid::Principal;

    use super::*;

    const TEST_PRINCIPAL: Principal = Principal::from_slice(&[
        0, 0, 0, 0, 0, 0, 0, 7, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    #[test]
    fn test_initial_subaccount() {
        let subaccount = Subaccount::default();
        assert_eq!(subaccount.environment(), Environment::Production);
        assert_eq!(subaccount.nonce(), 0);
        assert_eq!(subaccount.name(), "Default");
        assert_eq!(subaccount.id(), "-default");

        let identifier = AccountIdentifier::new(TEST_PRINCIPAL, subaccount);

        println!("{:?}", identifier.to_string());
    }

    #[test]
    fn test_subaccount() {
        let subaccount = Subaccount::new(Environment::Production, 1);
        println!("{:?}", subaccount);
        assert_eq!(subaccount.environment(), Environment::Production);
        assert_eq!(subaccount.nonce(), 1);
        assert_eq!(subaccount.name(), "Account 2");
        assert_eq!(subaccount.id(), "account_1");

        let subaccount = Subaccount::new(Environment::Staging, 1);
        assert_eq!(subaccount.environment(), Environment::Staging);
        assert_eq!(subaccount.nonce(), 1);
        assert_eq!(subaccount.name(), "Staging Account 2");
        assert_eq!(subaccount.id(), "staging_account_1");

        let subaccount = Subaccount::new(Environment::Development, 1);
        assert_eq!(subaccount.environment(), Environment::Development);
        assert_eq!(subaccount.nonce(), 1);
        assert_eq!(subaccount.name(), "Development Account 2");
        assert_eq!(subaccount.id(), "development_account_1");
    }

    #[test]
    fn test_subaccount_from_principal() {
        let subaccount = Subaccount::from(TEST_PRINCIPAL);

        println!("{:?}", subaccount);
        assert_eq!(subaccount.environment(), Environment::Production);
        assert_eq!(subaccount.nonce(), 0);
        assert_eq!(subaccount.name(), "Principal");
        assert_eq!(
            subaccount.id(),
            "principal_sqyeh-pqaaa-aaaaa-aaadq-caiaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaa"
        );
    }

    #[test]
    fn test_subaccount_derivation_path() {
        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]]
        );
        assert_eq!(subaccount.id(), "-default");
        assert_eq!(subaccount.name(), "Default");

        let subaccount = Subaccount::new(Environment::Production, 1);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1
            ]]
        );
        assert_eq!(subaccount.id(), "account_1");
        assert_eq!(subaccount.name(), "Account 2");

        let subaccount = Subaccount::new(Environment::Production, 255);
        assert_eq!(subaccount.environment(), Environment::Production);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 255
            ]]
        );
        assert_eq!(subaccount.id(), "account_255");
        assert_eq!(subaccount.name(), "Account 256");

        let subaccount = Subaccount::new(Environment::Staging, 512);
        assert_eq!(subaccount.environment(), Environment::Staging);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 0, 0,
                0, 0, 0, 2, 0
            ]]
        );
        assert_eq!(subaccount.id(), "staging_account_512");
        assert_eq!(subaccount.name(), "Staging Account 513");

        let subaccount = Subaccount::new(Environment::Development, 1024);
        assert_eq!(subaccount.environment(), Environment::Development);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 4, 0
            ]]
        );
        assert_eq!(subaccount.id(), "development_account_1024");
        assert_eq!(subaccount.name(), "Development Account 1025");
    }
}
