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
import { ethers, providers } from "ethers"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useState } from "react"
import { B3Wallet } from "service/actor"
import { AddressesWithChain } from "."
import TransferForm from "../TransferForm"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface EthCardProps extends AddressesWithChain {
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

const EthCard: React.FC<EthCardProps> = ({
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

  const handleEthTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ETH from ${from} to ${to}`)
      setLoading(true)

      const nonce = await provider.getTransactionCount(from)
      const gasPrice = await provider.getGasPrice().then(s => s.toHexString())
      const value = ethers.utils.parseEther(amount.toString()).toHexString()
      const data = "0x00"
      const gasLimit = ethers.BigNumber.from("24000").toHexString()
      const transaction = {
        nonce,
        gasPrice,
        gasLimit,
        to,
        value,
        data
      }

      try {
        const serializeTx = Buffer.from(
          ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
          "hex"
        )

        console.log(serializeTx)

        setLoading(false)

        console.log({ title: "Signing transaction...", variant: "subtle" })

        // const res = await actor.request_sign_transaction(
        //   id,
        //   [...serializeTx],
        //   97n
        // )
      } catch (error: any) {
        errorToast({
          title: "Error",
          description: error.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoading(false)
      }
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
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default EthCard
