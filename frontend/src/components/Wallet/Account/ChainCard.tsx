/* eslint-disable no-unused-vars */
import { DeleteIcon, InfoOutlineIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
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
import TransferForm from "./TransferForm"

import { keyframes } from "@chakra-ui/react"
import useToastMessage from "hooks/useToastMessage"

const pulse = keyframes`
  0% { transform: scale(1); color: orange; }
  50% { transform: scale(1.1); color: purple; }
  100% { transform: scale(1);  color: orange; }
`

interface AddressProps {
  actor: B3Wallet
  accountId: string
  symbol: ChainSymbol
  network: ChainNetwork
  chain: ChainEnum
  networkDetail: string
  address: string
  pendings: [BtcNetwork, string][]
  refetchAccount: () => void
  extra?: () => JSX.Element
}

const ChainCard: React.FC<AddressProps> = ({
  actor,
  symbol,
  address,
  accountId,
  networkDetail,
  network,
  chain,
  pendings,
  extra,
  refetchAccount,
  ...rest
}) => {
  const [balance, setBalances] = useState<bigint>()
  const [loading, setLoadings] = useState(false)
  const [ckbtcPending, setCkbtcPending] = useState<PendingTranscation>()

  const errorToast = useToastMessage()
  const toast = useToast()

  const handleBalance = useCallback(() => {
    setLoadings(true)
    actor
      .account_balance(accountId, chain)
      .then(res => {
        console.log(res)

        setBalances(res)
        setLoadings(false)
      })
      .catch(err => {
        errorToast({
          title: "Error",
          description: err.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoadings(false)
      })
  }, [actor, accountId])

  useEffect(handleBalance, [handleBalance])

  const handleTransfer = useCallback(
    async (chain: ChainEnum, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ${symbol} from ${accountId} to ${to}`)
      errorToast({
        title: `Sending ${symbol}`,
        description: `Transfering ${amount} ${symbol} from ${accountId} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setLoadings(true)

      await actor
        .account_send(accountId, chain, to, amount)
        .then(res => {
          console.log(res)

          setLoadings(false)
          handleBalance()
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

          setLoadings(false)
        })
    },
    [actor, handleBalance, accountId]
  )

  const showPendingToast = () => {
    toast({
      id: "ckbtc-pending",
      title: "ckBTC is being minted. Please wait for confirmations.",
      description: (
        <Box>
          <Text fontSize="sm">
            ckBTC({networkDetail}) has {ckbtcPending.currentConfirmations}{" "}
            confirmations. Please wait for {ckbtcPending.requiredConfirmations}{" "}
            confirmations before you can use it.
          </Text>
          <Stack direction="row" align="center">
            <Link
              variant="link"
              href={`https://blockstream.info/tx/${pendings[0][1]}`}
              target="_blank"
              rel="noreferrer"
            >
              View on Blockstream
            </Link>
            <Button
              onClick={() =>
                actor
                  .account_remove_pending_receive(
                    accountId,
                    network as BtcNetwork
                  )
                  .then(() => {
                    toast.closeAll()
                    handleBalance()
                  })
              }
            >
              Cancel
            </Button>
          </Stack>
        </Box>
      ),
      status: "loading",
      duration: 10000,
      isClosable: true
    })
  }

  const updatePending = useCallback(async () => {
    if (!pendings || !pendings.length) {
      return
    }
    actor
      .account_update_receive_pending(accountId, network as BtcNetwork)
      .then(utxos => {
        console.log(utxos)

        handleBalance()
        setCkbtcPending(undefined)
        toast.closeAll()
      })
      .catch(err => {
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

  const handleAddressRemove = async () => {
    setLoadings(true)

    actor
      .account_remove_address(accountId, chain)
      .then(() => {
        setLoadings(false)
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

        setLoadings(false)
      })
  }

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
                onClick={showPendingToast}
              />
            )}
            <IconButton
              aria-label="Refresh"
              icon={<RepeatIcon />}
              color="green"
              onClick={handleBalance}
            />
            <IconButton
              aria-label="Remove"
              onClick={handleAddressRemove}
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
          {extra && extra()}
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default ChainCard
