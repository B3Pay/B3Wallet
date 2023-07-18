use super::{
    btc::{btc::BtcChain, network::BtcNetwork},
    ckbtc::ckbtc::CkbtcChain,
    ecdsa::EcdsaPublicKey,
    error::LedgerError,
    evm::api::EvmChain,
    icp::icp::IcpChain,
    icrc::icrc::IcrcChain,
    types::{Balance, ChainId, PendingEnum, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{currency::TokenAmount, types::CanisterId, Subaccount};
use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    fn address(&self) -> String;
    fn pendings(&self) -> Vec<PendingEnum>;
    async fn balance(&self) -> Result<Balance, LedgerError>;
    async fn send(&self, to: String, amount: TokenAmount) -> Result<SendResult, LedgerError>;
    async fn send_mut(
        &mut self,
        to: String,
        amount: TokenAmount,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, LedgerError>;
    async fn check_pending(&self, pending_index: usize) -> Result<(), LedgerError>;
    fn add_pending(&mut self, pending: PendingEnum);
    fn remove_pending(&mut self, pending_index: usize);
    fn clear_pending(&mut self);
}

#[enum_dispatch(ChainTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Chain {
    CkbtcChain,
    IcrcChain,
    BtcChain,
    EvmChain,
    IcpChain,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::IcpChain(IcpChain::new(Subaccount::default()))
    }
}

impl Chain {
    pub async fn new_icrc_chain(
        canister_id: CanisterId,
        subaccount: Subaccount,
    ) -> Result<Self, LedgerError> {
        let icrc = IcrcChain::new(canister_id, subaccount)
            .await
            .map_err(LedgerError::IcrcError)?;
        let chain = Chain::IcrcChain(icrc);

        Ok(chain)
    }

    pub async fn new_ckbtc_chain(
        btc_network: BtcNetwork,
        subaccount: Subaccount,
    ) -> Result<Self, LedgerError> {
        let ckbtc = CkbtcChain::new(btc_network, subaccount)
            .await
            .map_err(LedgerError::CkbtcError)?;
        let chain = Chain::CkbtcChain(ckbtc);

        Ok(chain)
    }

    pub fn new_btc_chain(
        btc_network: BtcNetwork,
        subaccount: Subaccount,
        ecdsa_public_key: EcdsaPublicKey,
    ) -> Result<Self, LedgerError> {
        let address = ecdsa_public_key
            .btc_address(btc_network.into())?
            .to_string();

        let chain = Chain::BtcChain(BtcChain {
            min_confirmations: None,
            pendings: Vec::new(),
            ecdsa_public_key,
            btc_network,
            subaccount,
            address,
        });

        Ok(chain)
    }

    pub fn new_evm_chain(chain_id: ChainId, address: String) -> Self {
        Chain::EvmChain(EvmChain {
            chain_id,
            address,
            pendings: Vec::new(),
        })
    }

    pub fn new_icp_chain(subaccount: Subaccount) -> Self {
        Chain::IcpChain(IcpChain::new(subaccount))
    }

    pub fn icrc(&self) -> Result<IcrcChain, LedgerError> {
        match self {
            Chain::IcrcChain(icrc) => Ok(icrc.clone()),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn icrc_mut(&mut self) -> Result<&mut IcrcChain, LedgerError> {
        match self {
            Chain::IcrcChain(icrc) => Ok(icrc),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn ckbtc(&self) -> Result<CkbtcChain, LedgerError> {
        match self {
            Chain::CkbtcChain(ckbtc) => Ok(ckbtc.clone()),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn ckbtc_mut(&mut self) -> Result<&mut CkbtcChain, LedgerError> {
        match self {
            Chain::CkbtcChain(ckbtc) => Ok(ckbtc),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn btc(&self) -> Result<BtcChain, LedgerError> {
        match self {
            Chain::BtcChain(btc) => Ok(btc.clone()),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn btc_mut(&mut self) -> Result<&mut BtcChain, LedgerError> {
        match self {
            Chain::BtcChain(btc) => Ok(btc),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn evm(&self) -> Result<EvmChain, LedgerError> {
        match self {
            Chain::EvmChain(evm) => Ok(evm.clone()),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn evm_mut(&mut self) -> Result<&mut EvmChain, LedgerError> {
        match self {
            Chain::EvmChain(evm) => Ok(evm),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn icp(&self) -> Result<IcpChain, LedgerError> {
        match self {
            Chain::IcpChain(icp) => Ok(icp.clone()),
            _ => Err(LedgerError::InvalidChain),
        }
    }

    pub fn icp_mut(&mut self) -> Result<&mut IcpChain, LedgerError> {
        match self {
            Chain::IcpChain(icp) => Ok(icp),
            _ => Err(LedgerError::InvalidChain),
        }
    }
}
