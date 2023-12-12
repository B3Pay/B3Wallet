use crate::ledger::btc::address::BitcoinAddress;
use crate::ledger::btc::signature;
use crate::ledger::btc::tx::{hash160, SignedInput, SignedTransaction, TxSigHasher};
use crate::ledger::ckbtc::minter::Minter;
use crate::ledger::ecdsa::ECDSAPublicKey;
use crate::ledger::subaccount::SubaccountEcdsaTrait;
use crate::ledger::types::BtcPending;
use b3_utils::vec_to_hex_string;
use b3_utils::{ledger::ICRCAccount, Subaccount};
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::PublicKey;
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, UtxoFilter};
use ic_cdk::println;
use serde_bytes::ByteBuf;

use super::error::BitcoinError;
use super::network::BtcNetwork;
use super::tx::UnsignedTransaction;
use super::utxos::BtcUtxos;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct BtcChain {
    pub address: String,
    pub subaccount: Subaccount,
    pub btc_network: BtcNetwork,
    pub pendings: Vec<BtcPending>,
    pub ecdsa_public_key: ECDSAPublicKey,
    pub min_confirmations: Option<u32>,
}

impl BtcChain {
    /// Get the Bitcoin P2WPKH Address based on the public key.
    /// This is the address that the canister uses to send and receive funds.
    pub fn btc_address(&self) -> Result<BitcoinAddress, BitcoinError> {
        let address = BitcoinAddress::P2wshV0(self.ecdsa_public_key.0);

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
    /// This is the list of all the UTXOs that this Address has.
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
    ) -> Result<[u8; 32], BitcoinError> {
        let dst_address = BitcoinAddress::parse(&dst_address, self.btc_network)
            .map_err(|err| BitcoinError::InvalidAddress(err.to_string()))?;

        let own_address = self.btc_address()?;

        let utxo_res = self.get_utxos(None).await?;

        let utxo = BtcUtxos::try_from(utxo_res)?;

        let fee_rate = self.btc_network.fee_rate(49).await?;

        let unsigned_transaction =
            utxo.build_unsigned_transaction(&own_address, &dst_address, amount, fee_rate)?;

        let signed_transaction = self.sign_transaction(unsigned_transaction).await?;

        println!(
            "Signed transaction: {}",
            hex::encode(signed_transaction.serialize())
        );

        self.btc_network
            .send_transaction(&signed_transaction)
            .await?;

        let txid = signed_transaction.wtxid();

        Ok(txid)
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
    /// Gathers ECDSA signatures for all the inputs in the specified unsigned
    /// transaction.
    ///
    /// # Panics
    ///
    /// This function panics if the `output_account` map does not have an entry for
    /// at least one of the transaction previous output points.
    pub async fn sign_transaction(
        &self,
        unsigned_tx: UnsignedTransaction,
    ) -> Result<SignedTransaction, BitcoinError> {
        let mut signed_inputs = Vec::with_capacity(unsigned_tx.inputs.len());
        let sighasher = TxSigHasher::new(&unsigned_tx);

        for input in &unsigned_tx.inputs {
            let outpoint = &input.previous_output;

            let pubkey = ByteBuf::from(self.ecdsa_public_key.to_bytes());
            let pkhash = hash160(&pubkey);

            let sighash = sighasher.sighash(input, &pkhash);

            let sec1_signature = self
                .subaccount
                .sign_with_ecdsa(sighash.to_vec())
                .await
                .map_err(|err| BitcoinError::Signature(err.to_string()))?;

            signed_inputs.push(SignedInput {
                signature: signature::EncodedSignature::from_sec1(&sec1_signature),
                pubkey,
                previous_output: outpoint.clone(),
                sequence: input.sequence,
            });
        }

        Ok(SignedTransaction {
            inputs: signed_inputs,
            outputs: unsigned_tx.outputs,
            lock_time: unsigned_tx.lock_time,
        })
    }

    pub async fn swap_to_ckbtc(&self, amount: Satoshi) -> Result<BtcPending, BitcoinError> {
        let minter = Minter::new(self.btc_network.clone());

        let account = ICRCAccount::from(self.subaccount.clone());

        let dst_address = minter
            .get_btc_address(account.clone())
            .await
            .map_err(|err| BitcoinError::SwapToCkbtc(err.to_string()))?;

        let txid_bytes = self
            .transfer(dst_address, amount)
            .await
            .map_err(|err| BitcoinError::SwapToCkbtc(err.to_string()))?;

        let txid = vec_to_hex_string(txid_bytes);

        let pending = BtcPending {
            txid,
            account: account.to_string(),
        };

        Ok(pending)
    }
}
