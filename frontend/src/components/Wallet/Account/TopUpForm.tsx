import { Button, Input, Stack } from "@chakra-ui/react"
import React, { useCallback, useState } from "react"

interface TopUpFormProps {
  handleTopUp: (to: string, amount: bigint) => Promise<void>
}

const TopUpForm: React.FC<TopUpFormProps> = ({ handleTopUp }) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [loading, setLoading] = useState(false)

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Math.floor(Number(amount) * 10 ** decimals))

    setLoading(true)

    handleTopUp(to, bigintAmount)
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
  }, [amount, handleTopUp, to])

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
        Top Up
      </Button>
    </Stack>
  )
}

export default TopUpForm
