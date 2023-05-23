use super::types::{BtcTransaction, BtcUtxo};
use crate::{
    error::WalletError,
    ledger::types::{BtcTxId, BtcTxIn, BtcTxOut},
    utils::sec1_to_der,
};
use b3_helper::constants::GET_CURRENT_FEE_PERCENTILES_CYCLES;
use bitcoin::{
    absolute::LockTime, hashes::Hash, sighash::EcdsaSighashType, Address, OutPoint, Script,
    Sequence, Witness,
};
use ic_cdk::{
    api::{
        call::call_with_payment,
        management_canister::bitcoin::{
            BitcoinNetwork, GetCurrentFeePercentilesRequest, MillisatoshiPerByte, Satoshi,
        },
    },
    export::candid::Principal,
};

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

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

// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
pub async fn bitcoin_build_transaction(
    own_public_key: &[u8],
    own_address: &Address,
    own_utxos: &[BtcUtxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_byte: MillisatoshiPerByte,
) -> BtcTransaction {
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
        let mut transaction = bitcoin_build_transaction_with_fee(
            own_utxos,
            own_address,
            dst_address,
            amount,
            total_fee,
        )
        .expect("Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        for (_, input) in transaction.input.iter_mut().enumerate() {
            let mut sig_with_hashtype = sec1_to_der(vec![255; 64]);
            sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);

            // input.script_sig = ScriptBuf::new()
            //     .push_slice(&sig_with_hashtype)
            //     .push_slice(own_public_key)
            //     .into_script()
            //     .into();

            input.witness.clear();
        }

        let transaction_size = transaction.size();

        // Compute the fee based on the transaction size.
        let fee = fee_per_byte * transaction_size as u64;

        // If the fee is correct, we're done.
        if fee == total_fee {
            return transaction;
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
    own_utxos: &[BtcUtxo],
    own_address: &Address,
    dst_address: &Address,
    amount: u64,
    fee: u64,
) -> Result<BtcTransaction, String> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 1_000;

    // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
    // even if they were previously spent in a transaction. This isn't a
    // problem as long as at most one transaction is created per block and
    // we're using min_confirmations of 1.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in own_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            // We have enough inputs to cover the amount we want to spend.
            break;
        }
    }

    if total_spent < amount + fee {
        return Err(format!(
            "Insufficient balance: {}, trying to transfer {} satoshi with fee {}",
            total_spent, amount, fee
        ));
    }

    let inputs: Vec<BtcTxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| {
            let txid = BtcTxId::from_slice(&utxo.outpoint.txid[..]).unwrap();

            let previous_output = OutPoint::new(txid, utxo.outpoint.vout);
            let sequence = Sequence::MAX;
            let witness = Witness::new();
            let script_sig = Script::empty().into();

            BtcTxIn {
                previous_output,
                sequence,
                witness,
                script_sig,
            }
        })
        .collect();

    let mut outputs = vec![BtcTxOut {
        script_pubkey: dst_address.script_pubkey(),
        value: amount,
    }];

    let remaining_amount = total_spent - amount - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(BtcTxOut {
            script_pubkey: own_address.script_pubkey(),
            value: remaining_amount,
        });
    }

    Ok(BtcTransaction {
        input: inputs,
        output: outputs,
        lock_time: LockTime::ZERO,
        version: 1,
    })
}
