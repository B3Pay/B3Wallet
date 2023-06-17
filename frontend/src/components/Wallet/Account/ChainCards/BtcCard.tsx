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
  keyframes,
  useToast
} from "@chakra-ui/react"
import Address from "components/Wallet/Address"
import Balance from "components/Wallet/Balance"
import { BtcNetwork, ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import { PendingTranscation, extractConfirmations } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"
import { AddressesWithChain } from "."
import SwapForm from "../SwapForm"
import TransferForm from "../TransferForm"

const pulse = keyframes`
  0% { transform: scale(1); color: orange; }
  50% { transform: scale(1.1); color: purple; }
  100% { transform: scale(1);  color: orange; }
`

interface BtcCardProps extends AddressesWithChain {
  actor: B3Wallet | B3BasicWallet
  balance: bigint
  accountId: string
  balanceLoading: boolean
  transferLoading: boolean
  handleBalance: (chain: ChainEnum) => void
  handleTransfer: (
    chain: ChainEnum,
    to: string,
    amount: bigint
  ) => Promise<void>
  handleAddressRemove: (chain: ChainEnum) => void
}

const BtcCard: React.FC<BtcCardProps> = ({
  actor,
  chain,
  symbol,
  address,
  balance,
  pending,
  network,
  accountId,
  balanceLoading,
  transferLoading,
  networkDetail,
  handleBalance,
  handleTransfer,
  handleAddressRemove
}) => {
  const errorToast = useToastMessage()
  const [loading, setLoading] = useState(false)

  useEffect(() => handleBalance(chain), [actor, accountId])

  const swapBtcToCkbtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
      console.log(`Swapping ${amount} BTC to cKBTC on ${network}, to ${to}`)

      setLoading(true)

      actor
        .account_swap_btc_to_ckbtc(accountId, network, amount)
        .then(() => {
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

  const [ckbtcPending, setCkbtcPending] = useState<PendingTranscation>()

  const toast = useToast()

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
              href={`https://blockstream.info/tx/${pending[0][1]}`}
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
                    handleBalance(chain)
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

  const updatePending = useCallback(
    async (index: bigint) => {
      console.log("Updating pending", pending)
      actor
        .account_check_pending(accountId, chain, index)
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
    },
    [pending, chain]
  )

  useEffect(() => {
    if (!pending.length) {
      return
    }

    updatePending(0n)

    const interval = setInterval(() => {
      updatePending(0n)
    }, 60_000)

    return () => clearInterval(interval)
  }, [updatePending])

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
            {pending.length > 0 && (
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
              aria-label="Refresh"
              icon={<RepeatIcon />}
              color="green"
              onClick={() => handleBalance(chain)}
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
            loading={transferLoading}
            title={`Send ${symbol}`}
            handleTransfer={handleTransfer}
          />
          <SwapForm
            network={network as BtcNetwork}
            loading={loading}
            title="Swap BTC to ckBTC"
            handleSwap={swapBtcToCkbtc}
            noAddressInput
          />
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default BtcCard
