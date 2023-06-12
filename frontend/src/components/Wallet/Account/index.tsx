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
import { useCallback, useEffect, useMemo, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../../Loading"
import AccountTitle from "./AccountTitle"
import ChainCard from "./ChainCard"
import ChainsSelect from "./ChainSelect"

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
  pendings,
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

  const { errorToast } = useToastMessage()

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

  const getBtcBalance = useCallback(
    async (btcNetwork: BtcNetwork) => {
      setLoadings(prev => ({ ...prev, BTC: true }))

      actor
        .account_balance_btc(id, btcNetwork, [])
        .then(balance => {
          setBalances(prev => ({ ...prev, BTC: balance }))
          setLoadings(prev => ({ ...prev, BTC: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor, id, chains]
  )

  const getIcpBalance = useCallback(
    async (_: ChainNetwork) => {
      setLoadings(prev => ({ ...prev, ICP: true }))
      actor
        .account_icp_balance(id)
        .then(balance => {
          setBalances(prev => ({ ...prev, ICP: balance }))
          setLoadings(prev => ({ ...prev, ICP: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, id]
  )

  const getIcrcBalance = useCallback(
    async (canisterId: Principal) => {
      setLoadings(prev => ({ ...prev, ICRC: true }))

      actor
        .account_icrc_balance(id, canisterId)
        .then(balance => {
          setBalances(prev => ({ ...prev, ICRC: balance }))
          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
    },
    [actor, id]
  )

  const getCkbtcBalance = useCallback(
    async (btcNetwork: BtcNetwork) => {
      setLoadings(prev => ({ ...prev, ICRC: true }))

      actor
        .account_ckbtc_balance(id, btcNetwork)
        .then(balance => {
          setBalances(prev => ({ ...prev, ICRC: balance }))
          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
    },
    [actor, id]
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

        const res = await actor.request_sign_transaction(
          id,
          [...serializeTx],
          97n
        )

        console.log(res)
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

  const handleBtcTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} BTC from ${from} to ${to}`)
      setLoadings(prev => ({ ...prev, BTC: true }))

      await actor
        .account_send_btc(id, { Regtest: null }, to, amount)
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor, getBtcBalance, id]
  )

  const handleIcpTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ICP from ${from} to ${to}`)
      errorToast({
        title: "Sending ICP",
        description: `Transfering ${amount} ICP from ${from} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      const tokenAmount = {
        e8s: BigInt(amount)
      }

      setLoadings(prev => ({ ...prev, ICP: true }))

      await actor
        .account_send_icp(id, to, tokenAmount, [], [])
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, ICP: false }))

          errorToast({
            title: "Success",
            description: `Transfered ${amount} ICP from ${from} to ${to}`,
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

          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, getIcpBalance, id]
  )

  const handleTransfer = useCallback(
    async (chain: ChainEnum, to: string, amount: bigint) => {
      const symbol = Object.keys(chain)[0] as ChainSymbol

      console.log(`Transfering ${amount} ${symbol} from ${id} to ${to}`)
      errorToast({
        title: `Sending ${symbol}`,
        description: `Transfering ${amount} ${symbol} from ${id} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setLoadings(prev => ({ ...prev, CKBTC: true }))

      await actor
        .account_send(id, chain, to, amount)
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, CKBTC: false }))

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

          setLoadings(prev => ({ ...prev, CKBTC: false }))
        })
    },
    [actor, getCkbtcBalance, id]
  )

  const handleIcrcTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ICRC from ${from} to ${to}`)
      errorToast({
        title: "Sending ICRC",
        description: `Transfering ${amount} ICRC from ${from} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setLoadings(prev => ({ ...prev, ICRC: true }))
      await actor
        .account_send(
          id,
          { ICRC: Principal.fromText("be2us-64aaa-aaaaa-qaabq-cai") },
          to,
          BigInt(amount)
        )
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, ICRC: false }))
          errorToast({
            title: "Success",
            description: `Transfered ${amount} ICRC from ${from} to ${to}`,
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

          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
    },
    [actor, getIcrcBalance, id]
  )

  const handleBalance = useCallback(
    async (chain: ChainEnum) => {
      const symbol = Object.keys(chain)[0] as ChainSymbol

      setLoadings(prev => ({ ...prev, [symbol]: true }))
      actor
        .account_balance(id, chain)
        .then(res => {
          console.log(res)

          setBalances(prev => ({ ...prev, [symbol]: res }))
          setLoadings(prev => ({ ...prev, [symbol]: false }))
        })
        .catch(err => {
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, [symbol]: false }))
        })
    },
    [actor, id]
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

  useEffect(() => {
    if (!isExpanded && addressesWithChain.length > 0) {
      return
    }
    addressesWithChain.map(({ symbol, network }) => {
      switch (symbol) {
        case "ICP":
          getIcpBalance(network)
          break
        case "CKBTC":
          getIcpBalance(network)
          break
        case "ICRC":
          getIcrcBalance(network as Principal)
          break
        case "EVM":
          getEthBalance(network as bigint)
          break
        case "BTC":
          getBtcBalance(network as BtcNetwork)
          break
        default:
          console.log("Unknown chain symbol")
      }
    })
  }, [addressesWithChain, isExpanded])

  const handleAddressRemove = async (chain: ChainEnum) => {
    setLoadings(prev => ({ ...prev, global: true }))

    actor
      .account_remove_address(id, chain)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
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

        setLoadings(prev => ({ ...prev, global: false }))
      })
  }
  const handleTopup = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Toping up ${amount} ICP from ${from} to ${to}`)
      errorToast({
        title: "Toping up ICP",
        description: `Toping up ${amount} ICP from ${from} to ${to}`,
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
            description: `Toped up ${amount} ICP from ${from} to ${to}`,
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
    [actor, getIcpBalance, id, errorToast]
  )

  const swapBtcToCkbtc = useCallback(
    async (network: BtcNetwork, amount: bigint) => {
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
          <ChainsSelect
            account_id={id}
            actor={actor}
            refetchAccount={refetchAccount}
          />
          {addressesWithChain.map(({ symbol, ...rest }, index) => (
            <ChainCard
              {...rest}
              account_id={id}
              key={index}
              actor={actor}
              symbol={symbol}
              balance={balances[symbol]}
              loading={loadings[symbol]}
              pendings={pendings}
              handleBalance={handleBalance}
              handleTransfer={handleTransfer}
              handlerAddressRemove={handleAddressRemove}
              handleTopup={symbol === "ICP" ? handleTopup : undefined}
              handleSwapBtcToCkbtc={
                symbol === "BTC" ? swapBtcToCkbtc : undefined
              }
            />
          ))}
        </Stack>
      </AccordionPanel>
    </Box>
  )
}

export default Account
