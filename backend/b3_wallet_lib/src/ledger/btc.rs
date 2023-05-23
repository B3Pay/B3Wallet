use super::{
    network::BtcNetwork,
    types::{BtcOutPoint, BtcTransaction, BtcTxId, BtcUtxo},
    Ledger,
};
use crate::{error::WalletError, ledger::types::BtcTxOut, utils::sec1_to_der};
use b3_helper::constants::{
    GET_BALANCE_COST_CYCLES, GET_CURRENT_FEE_PERCENTILES_CYCLES, GET_UTXOS_COST_CYCLES,
};
use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    sighash::{EcdsaSighashType, SighashCache},
    Address, PublicKey, Script, TxIn, Txid, Witness,
};
use bitcoin::{consensus::serialize, psbt::PartiallySignedTransaction};
use candid::Principal;
use ic_cdk::api::{
    call::call_with_payment,
    management_canister::bitcoin::{
        bitcoin_get_current_fee_percentiles, bitcoin_send_transaction, BitcoinNetwork,
        GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse,
        MillisatoshiPerByte, Satoshi, SendTransactionRequest,
    },
};
use std::str::FromStr;

impl Ledger {
    pub async fn bitcoin_get_balance(
        &self,
        network: BitcoinNetwork,
    ) -> Result<Satoshi, WalletError> {
        let address = self.keys.get_btc_address(network.into())?;

        let (satoshi,): (Satoshi,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_balance",
            (GetBalanceRequest {
                address,
                network,
                min_confirmations: None,
            },),
            GET_BALANCE_COST_CYCLES,
        )
        .await
        .map_err(|err| WalletError::BitcoinGetBalanceError(err.1))?;

        Ok(satoshi)
    }

    pub async fn bitcoin_get_current_fee_percentiles(
        network: BitcoinNetwork,
    ) -> Result<Vec<MillisatoshiPerByte>, WalletError> {
        let (satoshies,): (Vec<MillisatoshiPerByte>,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_current_fee_percentiles",
            (GetCurrentFeePercentilesRequest { network },),
            GET_CURRENT_FEE_PERCENTILES_CYCLES,
        )
        .await
        .map_err(|err| WalletError::BitcoinGetCurrentFeePercentilesError(err.1))?;

        Ok(satoshies)
    }

    pub async fn bitcoin_get_utxos(
        &self,
        network: BitcoinNetwork,
    ) -> Result<GetUtxosResponse, WalletError> {
        let address = self.keys.get_btc_address(network.into())?;

        let (utxos,): (GetUtxosResponse,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_utxos",
            (GetUtxosRequest {
                address,
                network,
                filter: None,
            },),
            GET_UTXOS_COST_CYCLES,
        )
        .await
        .map_err(|err| WalletError::BitcoinGetUtxosError(err.1))?;

        Ok(utxos)
    }

    /// Sends a transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    pub async fn bitcoin_transfer(
        &self,
        network: BitcoinNetwork,
        amount: Satoshi,
        dst_address: String,
    ) -> Result<BtcTxId, WalletError> {
        let own_address = self.keys.get_btc_address(network.into())?;

        let _own_address = Address::from_str(&own_address)
            .unwrap()
            .require_network(bitcoin::Network::Bitcoin)
            .unwrap();

        let own_utxos = self.bitcoin_get_utxos(network).await?.utxos;
        // Get fee percentiles from previous transactions to estimate our own fee.
        let (fee_percentiles,) =
            bitcoin_get_current_fee_percentiles(GetCurrentFeePercentilesRequest { network })
                .await
                .map_err(|err| WalletError::BitcoinGetCurrentFeePercentilesError(err.1))?;

        let fee_per_byte = if fee_percentiles.is_empty() {
            // There are no fee percentiles. This case can only happen on a regtest
            // network where there are no non-coinbase transactions. In this case,
            // we use a default of 2000 millisatoshis/byte (i.e. 2 satoshi/byte)
            2000
        } else {
            // Choose the 50th percentile for sending fees.
            fee_percentiles[49]
        };

        let dst_address = Address::from_str(&dst_address)
            .unwrap()
            .require_network(bitcoin::Network::Bitcoin)
            .unwrap();

        // Build the transaction that sends `amount` to the destination address.
        let transaction =
            self.build_unsigned_transaction(&own_utxos, &dst_address, amount, fee_per_byte)?;

        let mut psbt = PartiallySignedTransaction::from_unsigned_tx(transaction.clone()).unwrap();

        // Sign the transaction.
        let signed_transaction = self.bitcoin_sign_transaction(&mut psbt).await?;

        let signed_transaction_bytes = serialize(&signed_transaction);

        let send_args = SendTransactionRequest {
            network,
            transaction: signed_transaction_bytes,
        };

        bitcoin_send_transaction(send_args)
            .await
            .map_err(|err| WalletError::BitcoinSendTransactionError(err.1))?;

        Ok(signed_transaction.txid())
    }

    // Sign a bitcoin transaction.
    //
    // IMPORTANT: This method is for demonstration purposes only and it only
    // supports signing transactions if:
    //
    // 1. All the inputs are referencing outpoints that are owned by `own_address`.
    // 2. `own_address` is a P2PKH address.
    async fn bitcoin_sign_transaction(
        &self,
        psbt: &mut PartiallySignedTransaction,
    ) -> Result<BtcTransaction, WalletError> {
        let own_public_key = self.keys.ecdsa()?;
        let pub_key = PublicKey::from_slice(&own_public_key).unwrap();

        let psbt_clone = psbt.clone();
        for (index, input) in psbt.inputs.iter_mut().enumerate() {
            let sig_cache = &mut SighashCache::new(&psbt_clone.unsigned_tx);

            let (message, _) = psbt_clone.sighash_ecdsa(index, sig_cache).unwrap();

            let message_bytes = message.as_ref().to_vec();

            let signature = self.sign_with_ecdsa(message_bytes).await?;

            let mut sig_with_hashtype = sec1_to_der(signature);
            sig_with_hashtype.push(EcdsaSighashType::All as u8);

            let script = Script::from_bytes(&sig_with_hashtype);

            let builder = Script::builder()
                .push_slice(script.script_hash())
                .push_key(&pub_key);

            input.final_script_sig = Some(builder.into_script());
            input.final_script_witness = Some(Witness::default());
        }

        Ok(psbt.clone().extract_tx())
    }

    pub fn build_unsigned_transaction(
        &self,
        utxos: &[BtcUtxo],
        recipient: &Address,
        amount: u64,
        fee_rate: u64,
    ) -> Result<BtcTransaction, WalletError> {
        let own_public_key = self.keys.ecdsa()?;
        let pub_key = PublicKey::from_slice(&own_public_key).unwrap();

        // We have a chicken-and-egg problem where we need to know the length
        // of the transaction in order to compute its proper fee, but we need
        // to know the proper fee in order to figure out the inputs needed for
        // the transaction.
        //
        // We solve this problem iteratively. We start with a fee of zero, build
        // and sign a transaction, see what its size is, and then update the fee,
        // rebuild the transaction, until the fee is set to the correct amount.
        let mut total_fee = 0;
        loop {
            let mut transaction = self
                .bitcoin_build_transaction_with_fee(utxos, recipient, amount, total_fee)
                .expect("Error building transaction.");

            // Sign the transaction. In this case, we only care about the size
            // of the signed transaction, so we use a mock signer here for efficiency.
            for (_, input) in transaction.input.iter_mut().enumerate() {
                let mut sig_with_hashtype = sec1_to_der(vec![255; 64]);

                sig_with_hashtype.push(EcdsaSighashType::All as u8);

                let script = Script::from_bytes(&sig_with_hashtype);

                let builder = Script::builder()
                    .push_slice(script.script_hash())
                    .push_key(&pub_key);

                input.script_sig = builder.into_script();
            }

            let transaction_size = transaction.strippedsize();

            // Compute the fee based on the transaction size.
            let fee = fee_rate * transaction_size as u64;

            // If the fee is correct, we're done.
            if fee == total_fee {
                return Ok(transaction);
            }

            // Otherwise, update the fee and try again.
            total_fee = fee;

            // If the fee is too high, we're done.
            if total_fee > amount {
                panic!("Fee is too high.");
            }
        }
    }

    pub fn bitcoin_build_transaction_with_fee(
        &self,
        own_utxos: &[BtcUtxo],
        dst_address: &Address,
        amount: u64,
        fee: u64,
    ) -> Result<BtcTransaction, WalletError> {
        // Assume that any amount below this threshold is dust.
        const DUST_THRESHOLD: u64 = 1_000;

        let mut tx = BtcTransaction {
            version: 2,
            input: Vec::new(),
            output: Vec::new(),
            lock_time: LockTime::ZERO,
        };

        // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
        // even if they were previously spent in a transaction. This isn't a
        // problem as long as at most one transaction is created per block and
        // we're using min_confirmations of 1.
        let mut total_spent = 0;
        for utxo in own_utxos.iter().rev() {
            total_spent += utxo.value;

            let mut tx_in = TxIn::default();

            let txid = Txid::from_slice(&utxo.outpoint.txid).unwrap();
            tx_in.previous_output = BtcOutPoint::new(txid, utxo.outpoint.vout);

            tx.input.push(tx_in);
            if total_spent >= amount + fee {
                // We have enough inputs to cover the amount we want to spend.
                break;
            }
        }

        // Check if the total value is sufficient
        if total_spent < amount + fee {
            return Err(WalletError::BitcoinInsufficientBalanceError(
                total_spent,
                amount,
            ));
        }

        tx.output.push(BtcTxOut {
            script_pubkey: dst_address.script_pubkey(),
            value: amount,
        });

        let remaining_amount = total_spent - amount - fee;

        if remaining_amount >= DUST_THRESHOLD {
            let address = self.keys.get_btc_address(BtcNetwork::Mainnet).unwrap();
            let own_address = Address::from_str(&address)
                .unwrap()
                .require_network(bitcoin::Network::Bitcoin)
                .unwrap();

            tx.output.push(BtcTxOut {
                script_pubkey: own_address.script_pubkey(),
                value: remaining_amount,
            });
        }

        Ok(tx)
    }
}

#[cfg(test)]
mod test {
    use super::super::types::BtcUtxo;
    use super::*;
    use b3_helper::types::{Environment, Subaccount};
    use ic_cdk::api::management_canister::bitcoin::Outpoint;

    #[test]
    fn test_build_unsigned_transaction() {
        let subaccount = Subaccount::new(Environment::Production, 0);

        let ledger = Ledger::from(subaccount);

        let utxos = vec![
            BtcUtxo {
                outpoint: Outpoint {
                    txid: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                    vout: 0,
                },
                value: 100_000_000,
                height: 0,
            },
            BtcUtxo {
                outpoint: Outpoint {
                    txid: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                    vout: 1,
                },
                value: 100_000_000,
                height: 1,
            },
        ];

        let recipient = Address::from_str("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq")
            .unwrap()
            .assume_checked();

        let transaction = ledger
            .build_unsigned_transaction(&utxos, &recipient, 100_000_000, 0)
            .unwrap();

        assert_eq!(transaction.input.len(), 2);

        assert_eq!(transaction.output.len(), 2);

        assert_eq!(transaction.output[0].value, 100_000_000);
    }
}
