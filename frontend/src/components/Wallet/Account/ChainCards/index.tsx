import { AccordionPanel, Skeleton, Stack } from "@chakra-ui/react"
import Loading from "components/Loading"
import {
  BtcPending,
  ChainEnum,
  CkbtcPending,
  IcpPending,
  PendingEnum,
  SendToken,
  TokenAmount
} from "declarations/b3_wallet/b3_wallet.did"
import { ChainNetwork, ChainSymbol, extractConfirmations } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useState } from "react"
import { B3Wallet } from "service"
import CreateAddress from "../CreateAddress"
import BtcCard from "./BtcCard"
import CkbtcCard from "./CkbtcCard"
import EthCard from "./EthCard"
import IcpCard from "./IcpCard"
import IcrcCard from "./IcrcCard"

export interface AddressesWithChain {
  id: string
  symbol: ChainSymbol
  pending: BtcPending[] | CkbtcPending[] | IcpPending[]
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
  pendings: Array<PendingEnum>
  actor: B3Wallet
  accountId: string
  isExpanded: boolean
  numberOfAddresses: number
  refetchAccount: () => void
}

const ChainCards: React.FC<ChainCardsProps> = ({
  numberOfAddresses,
  refetchAccount,
  isExpanded,
  accountId,
  addresses,
  actor
}) => {
  const [balances, setBalances] = useState({})
  const [balanceLoadings, setBalanceLoadings] = useState({})
  const [removeLoading, setRemoveLoading] = useState(false)

  const errorToast = useToastMessage()

  const handleBalance = useCallback(
    async (id: string, chain: ChainEnum) => {
      setBalanceLoadings(prev => ({ ...prev, [id]: true }))

      actor
        .account_balance(accountId, chain)
        .then(res => {
          console.log(res)

          setBalances(prev => ({ ...prev, [id]: res }))
          setBalanceLoadings(prev => ({ ...prev, [id]: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setBalanceLoadings(prev => ({ ...prev, [id]: false }))
        })
    },
    [actor, accountId]
  )

  const handleTransfer = useCallback(
    async (chain: ChainEnum, to: string, value: bigint, decimals = 8) => {
      let symbol = Object.keys(chain)[0]

      const valueInDecimal = Number(value) / 10 ** decimals

      if (value <= 0) {
        errorToast({
          title: "Error",
          description: "Amount must be greater than 0",
          status: "error",
          duration: 5000,
          isClosable: true
        })
        return
      }

      if (to === "") {
        errorToast({
          title: "Error",
          description: "Please enter a valid address",
          status: "error",
          duration: 5000,
          isClosable: true
        })
        return
      }

      console.log(
        `Transfering ${valueInDecimal} ${symbol} from ${accountId} to ${to}`
      )
      errorToast({
        title: `Sending ${symbol}`,
        description: `Transfering ${valueInDecimal} ${symbol} from ${accountId} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      let amount: TokenAmount = {
        amount: value,
        decimals
      }

      let sendArgs: SendToken = {
        account_id: accountId,
        chain,
        to,
        amount
      }

      try {
        let result
        let message: string
        if ("request_send" in actor) {
          message = "Transfer Requested"
          result = await actor.request_send(sendArgs, "Sending Test", [])
        } else {
          message = "Transfered"
          result = await actor.account_send(accountId, chain, to, amount)
        }

        console.log(result)

        errorToast({
          title: "Success",
          description: `${message} ${valueInDecimal} ${symbol} from ${
            Object.keys(chain)[0]
          } to ${to}`,
          status: "success",
          duration: 5000,
          isClosable: true
        })
      } catch (err) {
        errorToast({
          title: "Error",
          description: err.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      }
    },
    [actor, handleBalance, accountId]
  )

  const handleAddressRemove = async (chain: ChainEnum) => {
    setRemoveLoading(true)

    actor
      .account_remove_address(accountId, chain)
      .then(() => {
        setRemoveLoading(false)
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

        setRemoveLoading(false)
      })
  }

  const checkPending = useCallback(
    (id: string, chain: ChainEnum, index: bigint) => {
      setBalanceLoadings(prev => ({ ...prev, [id]: true }))
      actor
        .account_check_pending(accountId, chain, index)
        .then(() => {
          setBalanceLoadings(prev => ({ ...prev, [id]: false }))
          handleBalance(id, chain)
          refetchAccount()
        })
        .catch(err => {
          console.log(err)
          setBalanceLoadings(prev => ({ ...prev, [id]: false }))
          if (err.message.includes("Current confirmations:")) {
            let pending = extractConfirmations(err.message)

            console.log(pending)

            if (pending) {
              errorToast({
                title: "Error",
                description: `Transaction is still pending. Current confirmations: ${pending}`,
                status: "error",
                duration: 5000,
                isClosable: true
              })
            }
          }
        })
    },
    [handleBalance]
  )

  return (
    <AccordionPanel p={0} fontSize="14" position="relative">
      {removeLoading && <Loading title="Removing address" />}
      <CreateAddress
        accountId={accountId}
        actor={actor}
        refetchAccount={refetchAccount}
      />
      {isExpanded ? (
        <Stack pt={2}>
          {addresses.CKBTC?.map(({ id, ...addressProps }) => (
            <CkbtcCard
              id={id}
              key={id}
              handleAddressRemove={handleAddressRemove}
              balance={balances[id]}
              balanceLoading={balanceLoadings[id]}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              actor={actor}
              accountId={accountId}
              {...addressProps}
            />
          ))}
          {addresses.BTC?.map(({ id, ...addressProps }) => (
            <BtcCard
              id={id}
              key={id}
              checkPending={checkPending}
              refetchAccount={refetchAccount}
              handleAddressRemove={handleAddressRemove}
              balance={balances[id]}
              balanceLoading={balanceLoadings[id]}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              actor={actor}
              accountId={accountId}
              {...addressProps}
            />
          ))}
          {addresses.EVM?.map(({ id, ...addressProps }) => (
            <EthCard
              id={id}
              key={id}
              handleAddressRemove={handleAddressRemove}
              balance={balances[id]}
              balanceLoading={balanceLoadings[id]}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              actor={actor}
              accountId={accountId}
              {...addressProps}
            />
          ))}
          {addresses.ICRC?.map(({ id, ...addressProps }) => (
            <IcrcCard
              id={id}
              key={id}
              handleAddressRemove={handleAddressRemove}
              balance={balances[id]}
              balanceLoading={balanceLoadings[id]}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              actor={actor}
              accountId={accountId}
              {...addressProps}
            />
          ))}
          {addresses.ICP?.map(({ id, ...addressProps }) => (
            <IcpCard
              id={id}
              key={id}
              handleAddressRemove={handleAddressRemove}
              balance={balances[id]}
              balanceLoading={balanceLoadings[id]}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              actor={actor}
              accountId={accountId}
              {...addressProps}
            />
          ))}
        </Stack>
      ) : (
        <Stack pt={2}>
          {Array.from({ length: numberOfAddresses }).map((_, i) => (
            <Skeleton key={i} height="220px" borderRadius="lg" />
          ))}
        </Stack>
      )}
    </AccordionPanel>
  )
}

export default ChainCards
