use crate::ledger::ckbtc::minter::Minter;
use crate::ledger::ckbtc::types::BtcTxId;
use crate::ledger::ecdsa::EcdsaPublicKey;
use crate::ledger::subaccount::SubaccountTrait;
use crate::ledger::types::BtcPending;
use b3_helper_lib::{ICRCAccount, Subaccount};
use bitcoin::consensus::serialize;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::PublicKey;
use bitcoin::{ecdsa, hashes::Hash, Address, Script, Transaction};
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, UtxoFilter};
use ic_cdk::println;
use std::str::FromStr;

use super::error::BitcoinError;
use super::network::BtcNetwork;
use super::utxos::BtcUtxos;

use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct BtcChain {
    pub address: String,
    pub subaccount: Subaccount,
    pub btc_network: BtcNetwork,
    pub pendings: Vec<BtcPending>,
    pub ecdsa_public_key: EcdsaPublicKey,
    pub min_confirmations: Option<u32>,
}

impl BtcChain {
    /// Get the Bitcoin P2WPKH Address based on the public key.
    /// This is the address that the canister uses to send and receive funds.
    pub fn btc_address(&self) -> Result<Address, BitcoinError> {
        let address = self
            .ecdsa_public_key
            .btc_address(self.btc_network.into())
            .map_err(|err| BitcoinError::InvalidAddress(err.to_string()))?;

        Ok(address)
    }

    /// Get PublicKey from the ecdsa_public_key
    /// This is the public key that the canister uses to send and receive funds.
    pub fn btc_public_key(&self) -> Result<PublicKey, BitcoinError> {
        let public_key = self
            .ecdsa_public_key
            .btc_public_key()
            .map_err(|err| BitcoinError::InvalidPublicKey(err.to_string()))?;

        Ok(public_key)
    }

    /// Get the UTXOs of the canister's bitcoin wallet.
    /// This is the list of all the UTXOs that the canister owns.
    pub async fn get_utxos(
        &self,
        filter: Option<UtxoFilter>,
    ) -> Result<GetUtxosResponse, BitcoinError> {
        self.btc_network
            .get_utxos(self.address.clone(), filter)
            .await
    }

    /// Sends a transaction to the btc_network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    pub async fn transfer(
        &self,
        dst_address: String,
        amount: Satoshi,
    ) -> Result<BtcTxId, BitcoinError> {
        let dst_address = Address::from_str(&dst_address)
            .map_err(|err| BitcoinError::InvalidAddress(err.to_string()))?
            .require_network(self.btc_network.into())
            .map_err(|err| BitcoinError::InvalidNetworkAddress(err.to_string()))?;

        let own_address = self.btc_address()?;

        let utxo_res = self.get_utxos(None).await?;

        let utxo = BtcUtxos::try_from(utxo_res)?;

        let fee_rate = self.btc_network.fee_rate(49).await?;

        let mut tx = utxo.build_transaction(&own_address, &dst_address, amount, fee_rate)?;

        let signed_transaction = self.sign_transaction(&mut tx).await?;

        let signed_transaction_bytes = serialize(&signed_transaction);

        println!(
            "Signed transaction: {}",
            hex::encode(signed_transaction_bytes.clone())
        );

        self.btc_network
            .send_transaction(signed_transaction_bytes)
            .await?;

        let txid = signed_transaction.txid();

        Ok(txid.to_string())
    }

    /// Signs a message hash with the internet computer threshold signature.
    /// The message hash is signed with the internet computer threshold signature.
    async fn sign_btc_transaction(&self, message_hash: Vec<u8>) -> Result<Signature, BitcoinError> {
        let sig = self
            .subaccount
            .sign_with_ecdsa(message_hash)
            .await
            .map_err(|err| BitcoinError::Signature(err.to_string()))?;

        let signature = Signature::from_compact(&sig)
            .map_err(|err| BitcoinError::Signature(err.to_string()))?;

        Ok(signature)
    }

    /// Signs a bitcoin transaction.
    /// The transaction is signed with the internet computer threshold signature.
    async fn sign_transaction(
        &self,
        transaction: &mut Transaction,
    ) -> Result<Transaction, BitcoinError> {
        let address = self.btc_address()?;

        let public_key = self.btc_public_key()?;

        let sig_cache = SighashCache::new(transaction.clone());
        for (index, input) in transaction.input.iter_mut().enumerate() {
            let sign_hash = sig_cache
                .legacy_signature_hash(
                    index,
                    &address.script_pubkey(),
                    EcdsaSighashType::All.to_u32(),
                )
                .map_err(|err| BitcoinError::Signature(err.to_string()))?;

            let message_bytes = sign_hash.to_byte_array().to_vec();

            let signature = self
                .sign_btc_transaction(message_bytes)
                .await
                .map_err(|err| BitcoinError::Signature(err.to_string()))?;

            let sig = ecdsa::Signature::sighash_all(signature);

            let builder = Script::builder().push_slice(sig.serialize());

            input.script_sig = builder.push_key(&public_key).into_script();

            input.witness.clear();
        }

        Ok(transaction.clone())
    }

    pub async fn swap_to_ckbtc(&self, amount: Satoshi) -> Result<BtcPending, BitcoinError> {
        let minter = Minter::new(self.btc_network.clone());

        let account = ICRCAccount::from(self.subaccount.clone());

        let dst_address = minter
            .get_btc_address(account.clone())
            .await
            .map_err(|err| BitcoinError::SwapToCkbtc(err.to_string()))?;

        let txid = self
            .transfer(dst_address, amount)
            .await
            .map_err(|err| BitcoinError::SwapToCkbtc(err.to_string()))?;

        let pending = BtcPending {
            txid,
            account: account.to_string(),
        };

        Ok(pending)
    }
}
