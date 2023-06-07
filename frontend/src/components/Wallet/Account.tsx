/* eslint-disable no-unused-vars */
import { AccordionPanel, Box, Stack } from "@chakra-ui/react"

import { Principal } from "@dfinity/principal"
import {
  BtcNetwork,
  ChainType,
  WalletAccountView
} from "declarations/b3_wallet/b3_wallet.did"
import { ethers, providers } from "ethers"
import { isAddress } from "ethers/lib/utils"
import { ChainNetwork, ChainSymbol } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useMemo, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../Loading"
import { AccountTitle } from "./AccountTitle"
import { ChainCard } from "./ChainCard"
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
  detail: string
  address: string
  network: ChainNetwork
  chain: ChainType
}

const Account: React.FC<AccountProps> = ({
  actor,
  id,
  name,
  loading,
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

  const toast = useToastMessage()

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
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, EVM: false }))
        })
    },
    [toast]
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
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor, toast, id, chains]
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
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, toast, id]
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
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
    },
    [actor, toast, id]
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
        toast({
          title: "Error",
          description: error.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoadings(prev => ({ ...prev, EVM: false }))
      }
    },
    [actor, toast, getEthBalance, id]
  )

  const handleBtcTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} BTC from ${from} to ${to}`)
      setLoadings(prev => ({ ...prev, BTC: true }))

      await actor
        .request_transfer_btc(
          {
            account_id: id,
            to,
            network: { Regtest: null },
            amount
          },
          []
        )
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
        .catch(err => {
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor, toast, getBtcBalance, id]
  )

  const handleIcpTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ICP from ${from} to ${to}`)
      toast({
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

          toast({
            title: "Success",
            description: `Transfered ${amount} ICP from ${from} to ${to}`,
            status: "success",
            duration: 5000,
            isClosable: true
          })
        })
        .catch(err => {
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, toast, getIcpBalance, id]
  )

  const handleIcrcTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ICRC from ${from} to ${to}`)
      toast({
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
          toast({
            title: "Success",
            description: `Transfered ${amount} ICRC from ${from} to ${to}`,
            status: "success",
            duration: 5000,
            isClosable: true
          })
        })
        .catch(err => {
          toast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setLoadings(prev => ({ ...prev, ICRC: false }))
        })
    },
    [actor, toast, getIcrcBalance, id]
  )

  const handleTopup = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Toping up ${amount} ICP from ${from} to ${to}`)
      toast({
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

          toast({
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
    [actor, getIcpBalance, id, toast]
  )

  const handleTransfer = {
    EVM: handleEthTransfer,
    BTC: handleBtcTransfer,
    ICP: handleIcpTransfer,
    ICRC: handleIcrcTransfer
  }

  const handleBalance = {
    EVM: getEthBalance,
    BTC: getBtcBalance,
    ICP: getIcpBalance,
    ICRC: getIcrcBalance
  }

  const addressesWithChain: AddressesWithChain[] = useMemo(() => {
    const addressItem: AddressesWithChain[] = []

    chains.map(([chain, address]) => {
      const symbol = Object.keys(chain)[0] as ChainSymbol
      const network = Object.values(chain)[0] as ChainNetwork

      const detail =
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
        detail,
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

  const handleAddressRemove = async (chain: ChainType) => {
    setLoadings(prev => ({ ...prev, global: true }))

    actor
      .account_remove_address(id, chain)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
        refetchAccount()
      })
      .catch(e => {
        toast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoadings(prev => ({ ...prev, global: false }))
      })
  }

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
          {addressesWithChain.map(
            ({ address, network, chain, detail, symbol }, index) => (
              <ChainCard
                key={index}
                symbol={symbol}
                address={address}
                balance={balances[symbol]}
                detail={detail}
                network={network}
                loading={loadings[symbol]}
                handleBalance={handleBalance[symbol]}
                handleTransfer={handleTransfer[symbol]}
                handlerAddressRemove={() => handleAddressRemove(chain)}
                handleTopup={symbol === "ICP" ? handleTopup : undefined}
              />
            )
          )}
        </Stack>
      </AccordionPanel>
    </Box>
  )
}

export default Account
