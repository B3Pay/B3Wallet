import { DeleteIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  CardBody,
  CardHeader,
  Flex,
  Heading,
  IconButton,
  Stack,
  Text
} from "@chakra-ui/react"
import Address from "components/Wallet/Address"
import Balance from "components/Wallet/Balance"
import { BtcNetwork, ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import { AddressesWithChain } from "."
import SwapForm from "../SwapForm"
import TransferForm from "../TransferForm"

interface CkbtcCardProps extends AddressesWithChain {
  actor: B3Wallet
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

const CkbtcCard: React.FC<CkbtcCardProps> = ({
  actor,
  chain,
  symbol,
  address,
  balance,
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

  const swapCkbtcToBtc = useCallback(
    async (network: BtcNetwork, to: string, amount: bigint) => {
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
            <Address address={address} flex={9} />
            <Balance
              amount={balance}
              symbol={symbol}
              loading={balanceLoading}
              flex={3}
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
            title="Swap cKBTC to BTC"
            handleSwap={swapCkbtcToBtc}
          />
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default CkbtcCard
