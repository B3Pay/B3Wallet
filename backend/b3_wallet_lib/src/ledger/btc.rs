use super::{
    types::{BtcTransaction, BtcTxId},
    utils::{bitcoin_build_transaction, bitcoin_get_current_fee_percentiles, SIG_HASH_TYPE},
    Ledger,
};
use crate::{error::WalletError, utils::sec1_to_der};
use b3_helper::constants::{
    GET_BALANCE_COST_CYCLES, GET_UTXOS_COST_CYCLES, SEND_TRANSACTION_BASE_CYCLES,
    SEND_TRANSACTION_PER_BYTE_CYCLES,
};

use bitcoin::{script::Builder, Address};
use candid::Principal;
use ic_cdk::api::{
    call::call_with_payment,
    management_canister::bitcoin::{
        BitcoinNetwork, GetBalanceRequest, GetUtxosRequest, GetUtxosResponse, Satoshi,
        SendTransactionRequest,
    },
};
use std::str::FromStr;

impl Ledger {
    pub async fn bitcoin_send_transaction(
        &self,
        network: BitcoinNetwork,
        transaction: Vec<u8>,
    ) -> Result<(), WalletError> {
        let transaction_fee = SEND_TRANSACTION_BASE_CYCLES
            + (transaction.len() as u64) * SEND_TRANSACTION_PER_BYTE_CYCLES;

        call_with_payment(
            Principal::management_canister(),
            "bitcoin_send_transaction",
            (SendTransactionRequest {
                network,
                transaction,
            },),
            transaction_fee,
        )
        .await
        .map_err(|err| WalletError::BitcoinSendTransactionError(err.1))?;

        Ok(())
    }

    pub async fn bitcoin_get_balance(
        &self,
        network: BitcoinNetwork,
    ) -> Result<Satoshi, WalletError> {
        let address = self.public_keys.get_btc_address(network)?;

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

    pub async fn bitcoin_get_utxos(
        &self,
        network: BitcoinNetwork,
    ) -> Result<GetUtxosResponse, WalletError> {
        let address = self.public_keys.get_btc_address(network)?;

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
        let own_address = self.public_keys.get_btc_address(network)?;
        let own_public_key = self.public_keys.ecdsa()?;

        let own_utxos = self.bitcoin_get_utxos(network).await?.utxos;
        // Get fee percentiles from previous transactions to estimate our own fee.
        let fee_percentiles = bitcoin_get_current_fee_percentiles(network).await?;

        let fee_per_byte = if fee_percentiles.is_empty() {
            // There are no fee percentiles. This case can only happen on a regtest
            // network where there are no non-coinbase transactions. In this case,
            // we use a default of 2000 millisatoshis/byte (i.e. 2 satoshi/byte)
            2000
        } else {
            // Choose the 50th percentile for sending fees.
            fee_percentiles[49]
        };

        let own_address = Address::from_str(&own_address).unwrap();
        let dst_address = Address::from_str(&dst_address).unwrap();

        // Build the transaction that sends `amount` to the destination address.
        // let transaction = bitcoin_build_transaction(
        //     &own_public_key,
        //     &own_address,
        //     &own_utxos,
        //     &dst_address,
        //     amount,
        //     fee_per_byte,
        // )
        // .await;

        // Sign the transaction.
        // let signed_transaction = self.bitcoin_sign_transaction(network, transaction).await?;

        // let signed_transaction_bytes = signed_transaction.serialize();

        // self.bitcoin_send_transaction(network, signed_transaction_bytes)
        //     .await?;

        // Ok(signed_transaction.txid())

        todo!("bitcoin_transfer")
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
        network: BitcoinNetwork,
        mut transaction: BtcTransaction,
    ) -> Result<BtcTransaction, WalletError> {
        let own_public_key = self.public_keys.ecdsa()?;
        let address = self.public_keys.get_btc_address(network)?;

        let own_address = Address::from_str(&address).unwrap();

        let txclone = transaction.clone();
        for (index, input) in transaction.input.iter_mut().enumerate() {
            // let sighash = txclone.signature_hash(index, &own_public_key, SIG_HASH_TYPE.to_u32());

            // let signature = self.sign_with_ecdsa(sighash.to_vec()).await?;

            // let mut sig_with_hashtype = sec1_to_der(signature);

            // sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);

            // input.script_sig = Builder::new()
            //     .push_slice(sig_with_hashtype.as_slice())
            //     .push_slice(&own_public_key)
            //     .into_script();

            input.witness.clear();
        }

        Ok(transaction)
    }
}
