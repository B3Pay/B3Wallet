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
  keyframes,
  useToast
} from "@chakra-ui/react"
import Address from "components/Wallet/Address"
import Balance from "components/Wallet/Balance"
import { BtcNetwork, ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
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

  refetchAccount: () => void
  handleBalance: (id: string, chain: ChainEnum) => void
  handleTransfer: (
    chain: ChainEnum,
    to: string,
    amount: bigint
  ) => Promise<void>
  handleAddressRemove: (chain: ChainEnum) => void
}

const BtcCard: React.FC<BtcCardProps> = ({
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
  refetchAccount,
  handleAddressRemove
}) => {
  const errorToast = useToastMessage()
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    handleBalance(id, chain)
  }, [actor, accountId])

  const swapBtcToCkbtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
      console.log(`Swapping ${amount} BTC to cKBTC on ${network}, to ${to}`)

      setLoading(true)

      actor
        .account_swap_btc_to_ckbtc(accountId, network, amount)
        .then(() => {
          setLoading(false)
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

          setLoading(false)
        })
    },
    [actor]
  )

  const toast = useToast()

  const showPendingToast = useCallback(
    (amount: bigint) => {
      toast({
        title: "Pending",
        description: (
          <Text>
            You have {amount.toLocaleString()} BTC pending confirmation \nPlease
            wait for the transaction to be confirmed, txId:
            <Link
              variant="link"
              href={`https://blockstream.info/tx/${pending}`}
              target="_blank"
              rel="noreferrer"
            >
              View on Blockstream
            </Link>
          </Text>
        ),
        status: "warning",
        duration: 5000,
        isClosable: true
      })
    },
    [toast]
  )

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
            title="Swap to ckBTC"
            handleSwap={swapBtcToCkbtc}
            noAddressInput
          />
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default BtcCard
