import { AccordionPanel, Stack } from "@chakra-ui/react"
import { ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import { ChainNetwork, ChainSymbol } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useState } from "react"
import { B3Wallet } from "service/actor"
import CreateAddress from "../CreateAddress"
import BtcCard from "./BtcCard"
import EthCard from "./EthCard"
import IcpCard from "./IcpCard"

export interface AddressesWithChain {
  symbol: ChainSymbol
  networkDetail: string
  address: string
  network: ChainNetwork
  chain: ChainEnum
}

export type Addresses = {
  [key in ChainSymbol]?: AddressesWithChain[]
}

export interface Balances {
  EVM: bigint
  BTC: bigint
  ICP: bigint
  ICRC: bigint
  CKBTC: bigint
}

export interface Loadings {
  EVM: boolean
  BTC: boolean
  ICP: boolean
  ICRC: boolean
  CKBTC: boolean
}

interface ChainCardsProps {
  addresses: Addresses
  actor: B3Wallet
  accountId: string
  refetchAccount: () => void
}

const ChainCards: React.FC<ChainCardsProps> = ({
  actor,
  refetchAccount,
  accountId,
  addresses
}) => {
  const [balances, setBalances] = useState<Balances>({
    EVM: BigInt(0),
    BTC: BigInt(0),
    ICP: BigInt(0),
    ICRC: BigInt(0),
    CKBTC: BigInt(0)
  })
  const [balanceLoadings, setBalanceLoadings] = useState<Loadings>({
    EVM: false,
    BTC: false,
    ICP: false,
    ICRC: false,
    CKBTC: false
  })
  const [transferLoadings, setTransferLoadings] = useState<Loadings>({
    EVM: false,
    BTC: false,
    ICP: false,
    ICRC: false,
    CKBTC: false
  })
  const [globalLoading, setGlobalLoading] = useState(false)

  const errorToast = useToastMessage()

  const handleBalance = useCallback(
    (chain: ChainEnum) => {
      let symbol = Object.keys(chain)[0]

      setBalanceLoadings(prev => ({ ...prev, [symbol]: true }))

      actor
        .account_balance(accountId, chain)
        .then(res => {
          console.log(res)

          setBalances(prev => ({ ...prev, [symbol]: res }))
          setBalanceLoadings(prev => ({ ...prev, [symbol]: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setBalanceLoadings(prev => ({ ...prev, [symbol]: false }))
        })
    },
    [actor, accountId]
  )

  const handleTransfer = useCallback(
    async (chain: ChainEnum, to: string, amount: bigint) => {
      let symbol = Object.keys(chain)[0]

      console.log(`Transfering ${amount} ${symbol} from ${accountId} to ${to}`)
      errorToast({
        title: `Sending ${symbol}`,
        description: `Transfering ${amount} ${symbol} from ${accountId} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setTransferLoadings(prev => ({ ...prev, [symbol]: true }))

      await actor
        .account_send(accountId, chain, to, amount)
        .then(res => {
          console.log(res)

          setTransferLoadings(prev => ({ ...prev, [symbol]: false }))
          handleBalance(chain)
          errorToast({
            title: "Success",
            description: `Transfered ${amount} CKBTC from ${chain} to ${to}`,
            status: "success",
            duration: 5000,
            isClosable: true
          })
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setTransferLoadings(prev => ({ ...prev, [symbol]: false }))
        })
    },
    [actor, handleBalance, accountId]
  )

  const handleAddressRemove = async (chain: ChainEnum) => {
    setGlobalLoading(true)

    actor
      .account_remove_address(accountId, chain)
      .then(() => {
        setGlobalLoading(false)
        refetchAccount()
      })
      .catch(e => {
        errorToast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setGlobalLoading(false)
      })
  }

  return (
    <AccordionPanel px={0} fontSize="14">
      <Stack>
        <CreateAddress
          accountId={accountId}
          actor={actor}
          refetchAccount={refetchAccount}
        />
        {addresses.BTC?.map(addressProps => (
          <BtcCard
            key={addressProps.address}
            handleAddressRemove={handleAddressRemove}
            balance={balances.BTC}
            balanceLoading={balanceLoadings.BTC}
            transferLoading={transferLoadings.BTC}
            handleBalance={handleBalance}
            handleTransfer={handleTransfer}
            actor={actor}
            accountId={accountId}
            {...addressProps}
          />
        ))}
        {addresses.EVM?.map(addressProps => (
          <EthCard
            key={addressProps.address}
            handleAddressRemove={handleAddressRemove}
            balance={balances.EVM}
            balanceLoading={balanceLoadings.EVM}
            transferLoading={transferLoadings.EVM}
            handleBalance={handleBalance}
            handleTransfer={handleTransfer}
            actor={actor}
            accountId={accountId}
            {...addressProps}
          />
        ))}
        {addresses.ICP?.map(addressProps => (
          <IcpCard
            key={addressProps.address}
            handleAddressRemove={handleAddressRemove}
            balance={balances.ICP}
            balanceLoading={balanceLoadings.ICP}
            transferLoading={transferLoadings.ICP}
            handleBalance={handleBalance}
            handleTransfer={handleTransfer}
            actor={actor}
            accountId={accountId}
            {...addressProps}
          />
        ))}
        {addresses.ICRC?.map(addressProps => (
          <IcpCard
            key={addressProps.address}
            handleAddressRemove={handleAddressRemove}
            balance={balances.ICRC}
            balanceLoading={balanceLoadings.ICRC}
            transferLoading={transferLoadings.ICRC}
            handleBalance={handleBalance}
            handleTransfer={handleTransfer}
            actor={actor}
            accountId={accountId}
            {...addressProps}
          />
        ))}
      </Stack>
    </AccordionPanel>
  )
}

export default ChainCards
