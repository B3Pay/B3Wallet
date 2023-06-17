import { Button, Input, Stack } from "@chakra-ui/react"
import { ChainEnum } from "declarations/b3_wallet/b3_wallet.did"
import React, { useCallback, useState } from "react"

interface TransferFormProps {
  chain: ChainEnum
  title: string
  handleTransfer: (
    chain: ChainEnum,
    to: string,
    amount: bigint
  ) => Promise<void>
}

const TransferForm: React.FC<TransferFormProps> = ({
  chain,
  title,
  handleTransfer: handleTransfer
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [loading, setLoading] = useState(false)

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Math.floor(Number(amount) * 10 ** decimals))

    setLoading(true)

    await handleTransfer(chain, to, bigintAmount)
      .then(() => {
        setTo("")
        setAmount("")
      })
      .catch(e => {
        console.log(e)
      })
      .finally(() => {
        setLoading(false)
      })
  }, [chain, amount, handleTransfer, to])

  return (
    <Stack direction="row" justify="space-between" align="center">
      <Input
        id="to"
        alt="To"
        type="text"
        placeholder="To"
        flex={5}
        value={to}
        onChange={e => setTo(e.target.value)}
      />
      <Input
        id="amount"
        alt="Amount"
        placeholder="Amount"
        flex={4}
        type="text"
        value={amount}
        onChange={e => setAmount(e.target.value)}
      />
      <Button flex={3} onClick={transferHandler} isLoading={loading}>
        {title}
      </Button>
    </Stack>
  )
}

export default TransferForm
