use b3_utils::constants::{
    GET_BALANCE_COST_CYCLES, GET_CURRENT_FEE_PERCENTILES_CYCLES, GET_UTXOS_COST_CYCLES,
};
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::bitcoin::{bitcoin_send_transaction, SendTransactionRequest};
use ic_cdk::api::{
    call::call_with_payment,
    management_canister::bitcoin::{
        BitcoinNetwork, GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest,
        GetUtxosResponse, MillisatoshiPerByte, Satoshi, UtxoFilter,
    },
};
use serde::Serialize;
use std::fmt;

use crate::ledger::types::Balance;

use super::error::BitcoinError;
use super::tx::SignedTransaction;

/// Bitcoin Network.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub enum BtcNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

impl Default for BtcNetwork {
    fn default() -> Self {
        Self::Regtest
    }
}

impl BtcNetwork {
    /// Get the fee percentile.
    /// This is used to calculate the fee rate.
    pub async fn fee_percentiles(&self) -> Result<Vec<MillisatoshiPerByte>, BitcoinError> {
        let network = BitcoinNetwork::from(*self);

        let (satoshies,): (Vec<MillisatoshiPerByte>,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_current_fee_percentiles",
            (GetCurrentFeePercentilesRequest { network },),
            GET_CURRENT_FEE_PERCENTILES_CYCLES,
        )
        .await
        .map_err(|err| BitcoinError::GetCurrentFeePercentiles(err.1))?;

        Ok(satoshies)
    }

    /// Get the fee rate.
    /// This is used to calculate the fee amount.
    pub async fn fee_rate(&self, fee_percentile: u8) -> Result<MillisatoshiPerByte, BitcoinError> {
        let fee_percentiles = self.fee_percentiles().await?;

        if fee_percentiles.is_empty() {
            return Ok(2000);
        }

        let fee_percentile = fee_percentiles.get(fee_percentile as usize).ok_or(
            BitcoinError::InvalidFeePercentile(format!(
                "fee_percentile {} is out of range",
                fee_percentile
            )),
        )?;

        Ok(*fee_percentile)
    }

    /// Get the balance of the canister's bitcoin wallet.
    /// This is the sum of all the UTXOs that the canister owns.
    /// The balance is filtered by the given min_confirmations.
    /// If min_confirmations is None, then all UTXOs are considered.
    pub async fn get_balance(
        &self,
        address: String,
        min_confirmations: Option<u32>,
    ) -> Result<Balance, BitcoinError> {
        let network = BitcoinNetwork::from(*self);

        let (satoshi,): (Satoshi,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_balance",
            (GetBalanceRequest {
                address,
                network,
                min_confirmations,
            },),
            GET_BALANCE_COST_CYCLES,
        )
        .await
        .map_err(|err| BitcoinError::GetBalance(err.1))?;

        Ok(satoshi.into())
    }

    /// Get the UTXOs of the canister's bitcoin wallet.
    /// The UTXOs are filtered by the given filter.
    pub async fn get_utxos(
        &self,
        address: String,
        filter: Option<UtxoFilter>,
    ) -> Result<GetUtxosResponse, BitcoinError> {
        let network = BitcoinNetwork::from(*self);

        let (utxos,): (GetUtxosResponse,) = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_utxos",
            (GetUtxosRequest {
                address,
                network,
                filter,
            },),
            GET_UTXOS_COST_CYCLES,
        )
        .await
        .map_err(|err| BitcoinError::GetUtxos(err.1))?;

        Ok(utxos)
    }

    pub async fn send_transaction(
        &self,
        transaction: &SignedTransaction,
    ) -> Result<(), BitcoinError> {
        let network = BitcoinNetwork::from(*self);

        let send_args = SendTransactionRequest {
            network: network.into(),
            transaction: transaction.serialize(),
        };

        bitcoin_send_transaction(send_args)
            .await
            .map_err(|err| BitcoinError::SendTransaction(err.1))?;

        Ok(())
    }
}

impl From<BitcoinNetwork> for BtcNetwork {
    fn from(network: BitcoinNetwork) -> Self {
        match network {
            BitcoinNetwork::Mainnet => BtcNetwork::Mainnet,
            BitcoinNetwork::Testnet => BtcNetwork::Testnet,
            BitcoinNetwork::Regtest => BtcNetwork::Regtest,
        }
    }
}

impl From<bitcoin::Network> for BtcNetwork {
    fn from(network: bitcoin::Network) -> Self {
        match network {
            bitcoin::Network::Bitcoin => BtcNetwork::Mainnet,
            bitcoin::Network::Testnet => BtcNetwork::Testnet,
            bitcoin::Network::Regtest => BtcNetwork::Regtest,
            _ => panic!("Invalid network"),
        }
    }
}

impl From<BtcNetwork> for bitcoin::Network {
    fn from(network: BtcNetwork) -> Self {
        match network {
            BtcNetwork::Mainnet => bitcoin::Network::Bitcoin,
            BtcNetwork::Testnet => bitcoin::Network::Testnet,
            BtcNetwork::Regtest => bitcoin::Network::Regtest,
        }
    }
}
impl From<BtcNetwork> for BitcoinNetwork {
    fn from(network: BtcNetwork) -> Self {
        match network {
            BtcNetwork::Mainnet => BitcoinNetwork::Mainnet,
            BtcNetwork::Testnet => BitcoinNetwork::Testnet,
            BtcNetwork::Regtest => BitcoinNetwork::Regtest,
        }
    }
}

impl fmt::Display for BtcNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BtcNetwork::Mainnet => write!(f, "mainnet"),
            BtcNetwork::Testnet => write!(f, "testnet"),
            BtcNetwork::Regtest => write!(f, "regtest"),
        }
    }
}
