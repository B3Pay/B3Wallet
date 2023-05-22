use super::{
    network::BtcNetwork,
    types::{BtcOutPoint, BtcTransaction, BtcTxId, BtcUtxo},
    Ledger,
};
use crate::{error::WalletError, utils::sec1_to_der};
use b3_helper::constants::{
    GET_BALANCE_COST_CYCLES, GET_CURRENT_FEE_PERCENTILES_CYCLES, GET_UTXOS_COST_CYCLES,
};
use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    sighash::{EcdsaSighashType, SighashCache},
    Address, OutPoint, PublicKey, Script, Transaction, TxIn, TxOut, Txid, Witness,
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
        let address = self.public_keys.get_btc_address(network.into())?;

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
        let address = self.public_keys.get_btc_address(network.into())?;

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
        let own_address = self.public_keys.get_btc_address(network.into())?;
        let own_public_key = self.public_keys.ecdsa()?;

        let own_address = Address::from_str(&own_address)
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
        let own_public_key = self.public_keys.ecdsa()?;
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

    pub fn bitcoin_build_transaction(
        &self,
        inputs: Vec<(OutPoint, Address)>,
        outputs: Vec<(Address, u64)>,
    ) -> Result<PartiallySignedTransaction, Box<dyn std::error::Error>> {
        // Create an empty transaction
        let mut transaction = Transaction {
            version: 2,
            input: Vec::new(),
            output: Vec::new(),
            lock_time: LockTime::ZERO,
        };

        // Create inputs for the transaction
        for (outpoint, _) in &inputs {
            let mut txin = TxIn::default();

            txin.previous_output = *outpoint;

            transaction.input.push(txin);
        }

        // Create outputs for the transaction
        for (address, amount) in &outputs {
            let script_pubkey = address.script_pubkey();
            let txout = TxOut {
                script_pubkey,
                value: *amount,
            };
            transaction.output.push(txout);
        }

        // Create a PSBT from the transaction
        let mut psbt = PartiallySignedTransaction::from_unsigned_tx(transaction)?;

        // Fill in the necessary information for signing in the PSBT
        for (index, (_, address)) in inputs.iter().enumerate() {
            let pubkey = address.script_pubkey();
            psbt.inputs[index].witness_script = Some(pubkey);
            psbt.inputs[index].non_witness_utxo = None;
            psbt.inputs[index].sighash_type = Some(EcdsaSighashType::All.into());
        }

        // Return the built PSBT
        Ok(psbt)
    }

    pub fn build_unsigned_transaction(
        &self,
        utxos: &[BtcUtxo],
        recipient: &Address,
        amount: u64,
        fee: u64,
    ) -> Result<BtcTransaction, WalletError> {
        let mut tx = Transaction {
            version: 2,
            input: Vec::new(),
            output: Vec::new(),
            lock_time: LockTime::ZERO,
        };

        // Calculate the total value of the UTXOs
        let total_value: u64 = utxos.iter().map(|utxo| utxo.value).sum();

        // Check if the total value is sufficient
        if total_value < amount + fee {
            return Err(WalletError::BitcoinInsufficientBalanceError(
                total_value,
                amount,
            ));
        }

        // Add inputs to the transaction

        for utxo in utxos {
            let mut txin = TxIn::default();

            let txid = Txid::from_slice(&utxo.outpoint.txid).unwrap();

            txin.previous_output = BtcOutPoint::new(txid, utxo.outpoint.vout);

            tx.input.push(txin);
        }

        // Add output for the recipient
        let recipient_output = TxOut {
            value: amount,
            script_pubkey: recipient.script_pubkey(), // Assuming `recipient_script_pubkey` is the script pubkey of the recipient
        };

        tx.output.push(recipient_output);

        // Calculate the change value (total UTXO value - amount - fee)
        let change_value = total_value - amount - fee;

        let address = self.public_keys.get_btc_address(BtcNetwork::Mainnet)?;
        let own_address = Address::from_str(&address)
            .unwrap()
            .require_network(bitcoin::Network::Bitcoin)
            .unwrap();

        if change_value > 0 {
            let change_output = TxOut {
                value: change_value,
                script_pubkey: own_address.script_pubkey(), // Assuming `change_script_pubkey` is the script pubkey of the change address
            };
            tx.output.push(change_output);
        }

        Ok(tx)
    }
}
