pub mod network;
pub mod utils;
pub mod utxos;

use super::chains::Chains;
use super::{types::BtcTxId, Ledger};
use crate::error::WalletError;
use bitcoin::consensus::serialize;
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{ecdsa::Signature, hashes::Hash, Address, Script, Transaction};
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, UtxoFilter};
use network::BtcNetwork;
use std::str::FromStr;
use utxos::BtcUtxos;

impl Ledger {
    /// Get the balance of the canister's bitcoin wallet.
    /// This is the sum of all the UTXOs that the canister owns.
    pub async fn bitcoin_balance(
        &self,
        network: BtcNetwork,
        min_confirmations: Option<u32>,
    ) -> Result<Satoshi, WalletError> {
        let address = self.keys.get_address(Chains::BTC(network))?;

        network.get_balance(address, min_confirmations).await
    }

    /// Get the UTXOs of the canister's bitcoin wallet.
    /// This is the list of all the UTXOs that the canister owns.
    pub async fn bitcoin_get_utxos(
        &self,
        network: BtcNetwork,
        filter: Option<UtxoFilter>,
    ) -> Result<GetUtxosResponse, WalletError> {
        let address = self.keys.get_address(Chains::BTC(network))?;

        network.get_utxos(address, filter).await
    }

    /// Sends a transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    pub async fn bitcoin_transfer(
        &self,
        network: BtcNetwork,
        dst_address: &str,
        amount: Satoshi,
    ) -> Result<BtcTxId, WalletError> {
        let public_key = self.keys.get_public_key()?;

        let dst_address = Address::from_str(dst_address)
            .map_err(|_| WalletError::InvalidAddress)?
            .require_network(network.into())
            .map_err(|_| WalletError::InvalidNetworkAddress)?;

        let own_address = self.keys.get_btc_address(network)?;

        let utxo_res = network.get_utxos(own_address.to_string(), None).await?;

        let utxo = BtcUtxos::try_from(utxo_res)?;

        let fee_rate = network.fee_rate(49).await?;

        let mut tx =
            utxo.build_transaction(&public_key, &own_address, &dst_address, amount, fee_rate)?;

        let signed_transaction = self.bitcoin_sign_transaction(network, &mut tx).await?;

        let signed_transaction_bytes = serialize(&signed_transaction);

        network.send_transaction(signed_transaction_bytes).await?;

        Ok(signed_transaction.txid())
    }

    /// Signs a bitcoin transaction.
    async fn bitcoin_sign_transaction(
        &self,
        network: BtcNetwork,
        transaction: &mut Transaction,
    ) -> Result<Transaction, WalletError> {
        let public_key = self.keys.get_public_key()?;

        let address = Address::p2pkh(&public_key, network.into());

        let sig_cache = SighashCache::new(transaction.clone());
        for (index, input) in transaction.input.iter_mut().enumerate() {
            let sign_hash = sig_cache
                .legacy_signature_hash(
                    index,
                    &address.script_pubkey(),
                    EcdsaSighashType::All.to_u32(),
                )
                .map_err(|err| WalletError::BitcoinSignatureError(err.to_string()))?;

            let message_bytes = sign_hash.to_byte_array().to_vec();

            let signature = self.sign_btc_transaction(message_bytes).await?;

            let sig = Signature::sighash_all(signature);

            let builder = Script::builder().push_slice(sig.serialize());

            input.script_sig = builder.push_key(&public_key).into_script();

            input.witness.clear();
        }

        Ok(transaction.clone())
    }
}
