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
import { useEffect } from "react"
import { B3Wallet } from "service/actor"
import { AddressesWithChain } from "."
import TransferForm from "../TransferForm"

interface IcrcCardProps extends AddressesWithChain {
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

const IcrcCard: React.FC<IcrcCardProps> = ({
  actor,
  chain,
  symbol,
  address,
  balance,
  accountId,
  balanceLoading,
  transferLoading,
  networkDetail,
  handleBalance,
  handleTransfer,
  handleAddressRemove
}) => {
  useEffect(() => handleBalance(chain), [actor, accountId])

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
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default IcrcCard
