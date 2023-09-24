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
import { ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"
import { AddressesWithChain } from "."

interface EthCardProps extends AddressesWithChain {
  actor: B3Wallet
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

const EthCard: React.FC<EthCardProps> = ({
  id,
  actor,
  chain,
  symbol,
  address,
  balance,
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

  useEffect(() => {
    handleBalance(id, chain)
  }, [actor, accountId])

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
          <Text color="red.500" textAlign="center">
            Not available right now, please don't send any tokens to this
            address
          </Text>
          {/* <TransferForm
            chain={{ EVM: 80001n }}
            title={`Send ${symbol}`}
            handleTransfer={handleEthTransfer}
          /> */}
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default EthCard
