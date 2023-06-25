import {
  DeleteIcon,
  InfoOutlineIcon,
  RepeatClockIcon,
  RepeatIcon
} from "@chakra-ui/icons"
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
  keyframes,
  useToast
} from "@chakra-ui/react"
import Address from "components/Wallet/Address"
import Balance from "components/Wallet/Balance"
import { BtcNetwork, ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import { PendingTranscation, extractConfirmations } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"
import { AddressesWithChain } from "."
import SwapForm from "../SwapForm"
import TransferForm from "../TransferForm"

const pulse = keyframes`
  0% { transform: scale(1); color: orange; }
  50% { transform: scale(1.1); color: purple; }
  100% { transform: scale(1);  color: orange; }
`

interface CkbtcCardProps extends AddressesWithChain {
  actor: B3Wallet | B3BasicWallet
  balance: bigint
  accountId: string
  balanceLoading: boolean

  handleBalance: (id: string, chain: ChainEnum) => void
  handleTransfer: (
    chain: ChainEnum,
    to: string,
    amount: bigint
  ) => Promise<void>
  handleAddressRemove: (chain: ChainEnum) => void
}

const CkbtcCard: React.FC<CkbtcCardProps> = ({
  id,
  actor,
  chain,
  symbol,
  address,
  balance,
  pending,
  network,
  accountId,
  balanceLoading,

  networkDetail,
  handleBalance,
  handleTransfer,
  handleAddressRemove
}) => {
  const errorToast = useToastMessage()
  const [loading, setLoading] = useState(false)
  const [ckbtcPending, setCkbtcPending] = useState<PendingTranscation>()

  const toast = useToast()

  useEffect(() => {
    handleBalance(id, chain)
  }, [actor, accountId])

  const swapCkbtcToBtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
      if (amount <= 0) {
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

      console.log(`Swapping ${amount} cKBTC to BTC on ${network}, to ${to}`)
      setLoading(true)

      actor
        .account_swap_ckbtc_to_btc(accountId, network, to, amount)
        .then(txId => {
          console.log(txId)
          setLoading(false)
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

          setLoading(false)
        })
    },
    [actor]
  )

  const updateBalance = useCallback(() => {
    setLoading(true)

    actor
      .account_update_balance(accountId, network as BtcNetwork)
      .then(() => {
        setLoading(false)
        handleBalance(id, chain)
      })
      .catch(err => {
        console.log(err)
        if (err.message.includes("Current confirmations:")) {
          let pending = extractConfirmations(err.message)

          console.log(pending)

          if (pending.currentConfirmations > 0) {
            setCkbtcPending(pending)
          } else {
            setCkbtcPending(undefined)
          }

          setLoading(false)
          return
        }
      })
  }, [handleBalance, network])

  const showPendingToast = (index: bigint) => {
    if (!ckbtcPending) return
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
              href={`https://blockstream.info/tx/${pending}`}
              target="_blank"
              rel="noreferrer"
            >
              View on Blockstream
            </Link>
            <Button
              onClick={() =>
                actor
                  .account_remove_pending(accountId, chain, index)
                  .then(() => {
                    toast.closeAll()
                    handleBalance(id, chain)
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

  useEffect(() => {
    if (!ckbtcPending) {
      return
    }

    const interval = setInterval(() => {
      updateBalance()
    }, 60_000)

    return () => clearInterval(interval)
  }, [ckbtcPending, updateBalance])

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
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
                onClick={() => showPendingToast(0n)}
              />
            )}
            <IconButton
              aria-label="update-balance"
              icon={<RepeatClockIcon />}
              colorScheme="orange"
              variant="ghost"
              onClick={() => updateBalance()}
            />
            <IconButton
              aria-label="Refresh"
              icon={<RepeatIcon />}
              color="green"
              onClick={() => handleBalance(id, chain)}
            />
            <IconButton
              aria-label="Remove"
              onClick={() => handleAddressRemove(chain)}
              icon={<DeleteIcon />}
              color="red"
            />
          </Stack>
        </Stack>
      </CardHeader>
      <CardBody marginTop={0}>
        <Stack>
          <Stack direction="row" justify="space-between" align="center">
            <Address address={address} />
            <Balance
              amount={balance}
              symbol={symbol}
              loading={balanceLoading}
            />
          </Stack>
          <TransferForm
            chain={chain}
            title={`Send ${symbol}`}
            handleTransfer={handleTransfer}
          />
          <SwapForm
            network={network as BtcNetwork}
            title="Swap to BTC"
            handleSwap={swapCkbtcToBtc}
          />
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default CkbtcCard
