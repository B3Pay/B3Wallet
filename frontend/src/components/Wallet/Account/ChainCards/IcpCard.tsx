import { DeleteIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  CardBody,
  CardHeader,
  Flex,
  Heading,
  IconButton,
  Stack
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import Address from "components/Wallet/Address"
import Balance from "components/Wallet/Balance"
import { ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"
import { AddressesWithChain } from "."
import TopUpForm from "../TopUpForm"
import TransferForm from "../TransferForm"

interface IcpCardProps extends AddressesWithChain {
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

const IcpCard: React.FC<IcpCardProps> = ({
  id,
  actor,
  chain,
  symbol,
  address,
  balance,
  accountId,
  balanceLoading,

  networkDetail,
  handleBalance,
  handleTransfer,
  handleAddressRemove
}) => {
  const errorToast = useToastMessage()
  const [loadings, setLoadings] = useState(false)

  useEffect(() => {
    handleBalance(id, chain)
  }, [actor, accountId])

  const handleTopUp = useCallback(
    async (to: string, amount: bigint) => {
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

      console.log(`Toping up ${amount} ICP from ${accountId} to ${to}`)
      errorToast({
        title: "Toping up ICP",
        description: `Toping up ${amount} ICP from ${accountId} to ${to}`,
        status: "info",
        duration: 5000,
        isClosable: true
      })

      setLoadings(true)

      const tokens = {
        e8s: BigInt(amount)
      }

      const canister = Principal.fromText(to)

      await actor
        .account_top_up_and_notify(accountId, tokens, [canister])
        .then(res => {
          console.log(res)

          errorToast({
            title: "Success",
            description: `Toped up ${amount} ICP from ${accountId} to ${to}`,
            status: "success",
            duration: 5000,
            isClosable: true
          })

          setLoadings(false)
        })
        .catch(err => {
          console.log(err)
          setLoadings(false)
          errorToast({
            title: "Error",
            description: err.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })
        })
    },
    [actor, accountId, errorToast]
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
          <TransferForm
            chain={chain}
            title={`Send ${symbol}`}
            handleTransfer={handleTransfer}
          />
          <TopUpForm handleTopUp={handleTopUp} />
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default IcpCard
