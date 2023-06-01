/* eslint-disable no-unused-vars */
import { DeleteIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  Button,
  CardBody,
  CardHeader,
  Flex,
  Heading,
  IconButton,
  Input,
  Stack,
  Text
} from "@chakra-ui/react"
import React, { useCallback, useState } from "react"
import Address from "./Address"
import Balance from "./Balance"

interface AddressProps {
  balance: bigint
  chain: string
  address: string
  network: string
  handlerAddressRemove: (network: string, chain: string) => void
  handleTransfer: (from: string, to: string, amount: bigint) => Promise<void>
  handleBalance: () => Promise<void>
  loading: boolean
}

export const Chain: React.FC<AddressProps> = ({
  balance,
  chain,
  address,
  network,
  handlerAddressRemove,
  handleTransfer,
  handleBalance,
  loading,
  ...rest
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Number(amount) * 10 ** decimals)

    handleTransfer(address, to, bigintAmount)
      .then(() => {
        setTo("")
        setAmount("")
      })
      .catch(e => {
        console.log(e)
      })
  }, [address, amount, handleTransfer, to])

  const removeHandler = useCallback(async () => {
    handlerAddressRemove(chain, network)
  }, [handlerAddressRemove, network, chain])

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
            <Heading size="xs">{chain}</Heading>
          </Flex>
          <Flex flex={5}>
            <Text>{network}</Text>
          </Flex>
          <Stack direction="row" justify="end" align="center" flex={2}>
            <IconButton
              aria-label="Refresh"
              icon={<RepeatIcon />}
              color="green"
              onClick={handleBalance}
            />
            <IconButton
              aria-label="Remove"
              onClick={removeHandler}
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
              chain={chain}
              loading={loading}
              flex={3}
            />
          </Stack>
          <Stack direction="row" justify="space-between" align="center">
            <Input
              id="to"
              alt="To"
              type="text"
              placeholder="To"
              style={{
                flex: 5
              }}
              value={to}
              onChange={e => setTo(e.target.value)}
            />
            <Input
              id="amount"
              alt="Amount"
              placeholder="Amount"
              style={{
                flex: 4
              }}
              type="text"
              value={amount}
              onChange={e => setAmount(e.target.value)}
            />
            <Button
              style={{
                flex: 3
              }}
              onClick={transferHandler}
              isLoading={loading}
            >
              Send {chain}
            </Button>
          </Stack>
        </Stack>
      </CardBody>
    </Stack>
  )
}
