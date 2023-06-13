/* eslint-disable no-unused-vars */
import { AccordionPanel, Box, Stack } from "@chakra-ui/react"

import { Principal } from "@dfinity/principal"
import {
  BtcNetwork,
  ChainEnum,
  WalletAccountView
} from "declarations/b3_wallet/b3_wallet.did"
import { ethers, providers } from "ethers"
import { isAddress } from "ethers/lib/utils"
import { ChainNetwork, ChainSymbol } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useMemo, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../../Loading"
import AccountTitle from "./AccountTitle"
import ChainCard from "./ChainCard"
import CreateAddress from "./CreateAddress"
import SwapForm from "./SwapForm"
import TopUpForm from "./TopUpForm"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface AccountProps extends WalletAccountView {
  actor: B3Wallet
  loading: boolean
  isExpanded: boolean
  refetchAccount: () => void
}

export interface Balances {
  EVM: bigint
  BTC: bigint
  ICP: bigint
  ICRC: bigint
}

export interface Loadings {
  global: boolean
  EVM: boolean
  BTC: boolean
  ICP: boolean
  ICRC: boolean
}

export type AddressesWithChain = {
  symbol: ChainSymbol
  networkDetail: string
  address: string
  network: ChainNetwork
  chain: ChainEnum
}

const Account: React.FC<AccountProps> = ({
  actor,
  id,
  name,
  loading,
  pending_receive,
  addresses: chains,
  isExpanded,
  environment,
  refetchAccount
}) => {
  const [loadings, setLoadings] = useState<Loadings>({
    global: false,
    EVM: false,
    BTC: false,
    ICP: false,
    ICRC: false
  })
  const [balances, setBalances] = useState<Balances>({
    EVM: 0n,
    BTC: 0n,
    ICP: 0n,
    ICRC: 0n
  })

  const errorToast = useToastMessage()

  const getEthBalance = useCallback(
    async (chainId: bigint) => {
      const address = ""
      if (isAddress(address) === false) {
        return
      }

      setLoadings(prev => ({ ...prev, EVM: true }))
      provider
        .getBalance(address)
        .then(balance => {
          setBalances(prev => ({ ...prev, ETH: balance.toBigInt() }))
          setLoadings(prev => ({ ...prev, EVM: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, EVM: false }))
        })
    },
    [errorToast]
  )

  const handleEthTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ETH from ${from} to ${to}`)
      setLoadings(prev => ({ ...prev, EVM: true }))

      const nonce = await provider.getTransactionCount(from)
      const gasPrice = await provider.getGasPrice().then(s => s.toHexString())
      const value = ethers.utils.parseEther(amount.toString()).toHexString()
      const data = "0x00"
      const gasLimit = ethers.BigNumber.from("24000").toHexString()
      const transaction = {
        nonce,
        gasPrice,
        gasLimit,
        to,
        value,
        data
      }

      try {
        const serializeTx = Buffer.from(
          ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
          "hex"
        )

        console.log(serializeTx)

        setLoadings(prev => ({ ...prev, EVM: false }))

        console.log({ title: "Signing transaction...", variant: "subtle" })

        // const res = await actor.request_sign_transaction(
        //   id,
        //   [...serializeTx],
        //   97n
        // )
      } catch (error: any) {
        errorToast({
          title: "Error",
          description: error.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoadings(prev => ({ ...prev, EVM: false }))
      }
    },
    [actor, getEthBalance, id]
  )

  const addressesWithChain: AddressesWithChain[] = useMemo(() => {
    const addressItem: AddressesWithChain[] = []

    chains.map(([chain, address]) => {
      const symbol = Object.keys(chain)[0] as ChainSymbol
      const network = Object.values(chain)[0] as ChainNetwork

      const networkDetail =
        network === null
          ? null
          : typeof network === "string"
          ? network
          : typeof network === "bigint"
          ? network.toString()
          : (network as Principal)._isPrincipal
          ? network.toString()
          : Object.keys(network)[0]

      addressItem.push({
        address,
        symbol,
        network,
        networkDetail,
        chain
      })
    })

    return addressItem
  }, [chains])

  const handleTopUp = useCallback(
    async (to: string, amount: bigint) => {
      console.log(`Toping up ${amount} ICP from ${id} to ${to}`)
      errorToast({
        title: "Toping up ICP",
        description: `Toping up ${amount} ICP from ${id} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setLoadings(prev => ({ ...prev, ICP: true }))

      const tokens = {
        e8s: BigInt(amount)
      }

      const canister = Principal.fromText(to)

      await actor
        .account_top_up_and_notify(id, tokens, [canister], [])
        .then(res => {
          console.log(res)

          errorToast({
            title: "Success",
            description: `Toped up ${amount} ICP from ${id} to ${to}`,
            status: "success",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICP: false }))
        })
        .catch(err => {
          console.log(err)
          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, id, errorToast]
  )

  const swapBtcToCkbtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
      console.log(`Swapping ${amount} BTC to cKBTC on ${network}, to ${to}`)

      setLoadings(prev => ({ ...prev, BTC: true }))

      actor
        .account_swap_btc_to_ckbtc(id, network, amount)
        .then(() => {
          setLoadings(prev => ({ ...prev, BTC: false }))
          refetchAccount()
        })
        .catch(e => {
          console.log(e)
          errorToast({
            title: "Error",
            description: e.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor]
  )

  const swapCkbtcToBtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
      console.log(`Swapping ${amount} cKBTC to BTC on ${network}, to ${to}`)
      setLoadings(prev => ({ ...prev, CKBTC: true }))

      actor
        .account_swap_ckbtc_to_btc(id, network, to, amount)
        .then(txId => {
          console.log(txId)
          setLoadings(prev => ({ ...prev, CKBTC: false }))
          refetchAccount()
        })
        .catch(e => {
          console.log(e)
          errorToast({
            title: "Error",
            description: e.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, CKBTC: false }))
        })
    },
    [actor]
  )

  return (
    <Box position="relative">
      {(loadings.global || loading) && <Loading />}
      <AccountTitle
        id={id}
        name={name}
        actor={actor}
        environment={environment}
        setLoadings={setLoadings}
        refetchAccount={refetchAccount}
      />
      <AccordionPanel px={0} fontSize="14">
        <Stack spacing="2">
          <CreateAddress
            account_id={id}
            actor={actor}
            refetchAccount={refetchAccount}
          />
          {isExpanded &&
            addressesWithChain.map(({ symbol, network, ...rest }, index) => (
              <ChainCard
                {...rest}
                accountId={id}
                key={index}
                network={network}
                actor={actor}
                symbol={symbol}
                pendings={pending_receive}
                refetchAccount={refetchAccount}
                extra={() => {
                  switch (symbol) {
                    case "ICP":
                      return (
                        <TopUpForm
                          loading={loading}
                          handleTopUp={handleTopUp}
                        />
                      )
                    case "BTC":
                      return (
                        <SwapForm
                          network={network as BtcNetwork}
                          loading={loading}
                          toAddress={rest.address}
                          title="Swap BTC to cKBTC"
                          handleSwap={swapBtcToCkbtc}
                        />
                      )
                    case "CKBTC":
                      return (
                        <SwapForm
                          network={network as BtcNetwork}
                          loading={loading}
                          title="Swap cKBTC to BTC"
                          handleSwap={swapCkbtcToBtc}
                        />
                      )
                    default:
                      return null
                  }
                }}
              />
            ))}
        </Stack>
      </AccordionPanel>
    </Box>
  )
}

export default Account
