/* eslint-disable no-unused-vars */
import { DeleteIcon, InfoOutlineIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  CardBody,
  CardHeader,
  Flex,
  Heading,
  IconButton,
  Link,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import { BtcNetwork, ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import {
  ChainNetwork,
  ChainSymbol,
  PendingTranscation,
  extractConfirmations
} from "helpers/utiles"
import React, { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import Address from "../Address"
import Balance from "../Balance"
import SwapForm from "./SwapForm"
import TopUpForm from "./TopUpForm"
import TransferForm from "./TransferForm"

import { keyframes } from "@chakra-ui/react"

const pulse = keyframes`
  0% { transform: scale(1); color: orange; }
  50% { transform: scale(1.1); color: purple; }
  100% { transform: scale(1);  color: orange; }
`

interface AddressProps {
  actor: B3Wallet
  account_id: string
  balance: bigint
  symbol: ChainSymbol
  network: ChainNetwork
  chain: ChainEnum
  networkDetail: string
  address: string
  pendings?: [ChainEnum, string][]
  handlerAddressRemove: (chain: ChainEnum) => void
  handleTransfer: (
    chain: ChainEnum,
    to: string,
    amount: bigint
  ) => Promise<void>
  handleTopup?: (from: string, to: string, amount: bigint) => Promise<void>
  handleSwapBtcToCkbtc?: (network: BtcNetwork, amount: bigint) => Promise<void>
  handleBalance: (chain: ChainEnum) => Promise<void>
  loading: boolean
}

const ChainCard: React.FC<AddressProps> = ({
  balance,
  actor,
  symbol,
  address,
  account_id,
  networkDetail,
  network,
  chain,
  pendings,
  handleSwapBtcToCkbtc,
  handlerAddressRemove,
  handleTransfer,
  handleBalance,
  handleTopup,
  loading,
  ...rest
}) => {
  const [ckbtcPending, setCkbtcPending] = useState<PendingTranscation>()

  const toast = useToast()

  const showPendingToast = () => {
    toast({
      id: "ckbtc-pending",
      title: "ckBTC is being minted. Please wait for confirmations.",
      description: (
        <Stack>
          <Text>
            ckBTC({networkDetail}) has {ckbtcPending.currentConfirmations}{" "}
            confirmations. Please wait for {ckbtcPending.requiredConfirmations}{" "}
            confirmations before you can use it.
          </Text>
          <Link
            variant="link"
            href={`https://blockstream.info/tx/${pendings[0][1]}`}
            target="_blank"
            rel="noreferrer"
          >
            View on Blockstream
          </Link>
        </Stack>
      ),
      status: "info",
      duration: 10000,
      isClosable: true
    })
  }

  const updatePending = useCallback(async () => {
    if (!pendings || !pendings.length) {
      return
    }
    actor
      .account_update_balance(account_id, network as BtcNetwork)
      .then(utxos => {
        console.log(utxos)

        handleBalance(chain)
        setCkbtcPending(undefined)
        toast.closeAll()
      })
      .catch(err => {
        console.log(err)
        if (err.message.includes("Current confirmations:")) {
          let pending = extractConfirmations(err.message)

          console.log(pending)

          if (pending.currentConfirmations === null) {
            setCkbtcPending(undefined)
          } else {
            setCkbtcPending(pending)
          }
          return
        }
      })
  }, [pendings])

  useEffect(() => {
    if (symbol !== "CKBTC" || !pendings.length) {
      return
    }

    updatePending()

    const interval = setInterval(() => {
      updatePending()
    }, 60_000)

    return () => clearInterval(interval)
  }, [updatePending])

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      {...rest}
    >
      <CardHeader pb={0}>
        <Stack direction="row" justify="space-between" align="center">
          <Flex flex={5}>
            <Heading size="xs">{symbol}</Heading>
          </Flex>
          <Flex flex={5}>
            <Text>{networkDetail}</Text>
          </Flex>
          <Stack direction="row" justify="end" align="center" flex={2}>
            {ckbtcPending && (
              <IconButton
                aria-label="btc-pending"
                icon={<InfoOutlineIcon />}
                colorScheme="orange"
                animation={`${pulse} 1s infinite`}
                variant="ghost"
                onClick={() => showPendingToast()}
              />
            )}
            <IconButton
              aria-label="Refresh"
              icon={<RepeatIcon />}
              color="green"
              onClick={() => handleBalance(chain)}
            />
            <IconButton
              aria-label="Remove"
              onClick={() => handlerAddressRemove(chain)}
              icon={<DeleteIcon />}
              color="red"
            />
          </Stack>
        </Stack>
      </CardHeader>
      <CardBody marginTop={0}>
        <Stack>
          <Stack direction="row" justify="space-between" align="center">
            <Address address={address} flex={9} />
            <Balance
              amount={balance}
              symbol={symbol}
              loading={loading}
              flex={3}
            />
          </Stack>
          <TransferForm
            chain={chain}
            loading={loading}
            title={`Send ${symbol}`}
            handleTransfer={handleTransfer}
          />
          {handleTopup && (
            <TopUpForm
              address={address}
              loading={loading}
              title="Topup"
              handleTransfer={handleTopup}
            />
          )}
          {handleSwapBtcToCkbtc && (
            <SwapForm
              network={network as BtcNetwork}
              loading={loading}
              title="Swap BTC to cKBTC"
              handleSwap={handleSwapBtcToCkbtc}
            />
          )}
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default ChainCard
