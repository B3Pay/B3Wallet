use super::{
    btc::{btc::BtcChain, network::BtcNetwork},
    ckbtc::ckbtc::CkbtcChain,
    ecdsa::EcdsaPublicKey,
    error::LedgerError,
    icp::icp::IcpChain,
    icrc::types::IcrcChain,
    types::{Balance, ChainId, EvmChain, Pendings, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{subaccount::Subaccount, types::CanisterId};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    fn address(&self) -> String;
    fn pendings(&self) -> Pendings;
    async fn balance(&self) -> Result<Balance, LedgerError>;
    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError>;
    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, LedgerError>;
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
            pending: Vec::new(),
            ecdsa_public_key,
            btc_network,
            subaccount,
            address,
        });

        Ok(chain)
    }

    pub fn new_evm_chain(chain_id: ChainId, address: String) -> Self {
        Chain::EvmChain(EvmChain { chain_id, address })
    }

    pub fn new_icp_chain(subaccount: Subaccount) -> Self {
        Chain::IcpChain(IcpChain::new(subaccount))
    }

    pub fn has_pending_send(&self, txid: &String) -> bool {
        self.ckbtc()
            .map(|ckbtc| ckbtc.has_pending(txid))
            .unwrap_or(false)
    }

    pub fn has_pending_receive(&self, block_index: &String) -> bool {
        self.btc()
            .map(|ckbtc| ckbtc.has_pending(block_index))
            .unwrap_or(false)
    }

    pub fn add_pending_receive(&mut self, txid: String) -> Result<(), LedgerError> {
        let ckbtc = self.ckbtc_mut()?;

        ckbtc.add_pending(txid);

        Ok(())
    }

    pub fn remove_pending_receive(&mut self, txid: &String) -> Result<(), LedgerError> {
        let ckbtc = self.ckbtc_mut()?;

        ckbtc.remove_pending(txid);

        Ok(())
    }

    pub fn add_pending_send(&mut self, block_index: String) -> Result<(), LedgerError> {
        let btc = self.btc_mut()?;

        btc.add_pending(block_index);

        Ok(())
    }

    pub fn remove_pending_send(&mut self, block_index: String) -> Result<(), LedgerError> {
        let btc = self.btc_mut()?;

        btc.remove_pending(&block_index);

        Ok(())
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

#[async_trait]
impl ChainTrait for EvmChain {
    fn address(&self) -> String {
        let address = self.address.clone();

        address
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for BTC...")
    }

    fn pendings(&self) -> Pendings {
        Pendings::new()
    }
}
