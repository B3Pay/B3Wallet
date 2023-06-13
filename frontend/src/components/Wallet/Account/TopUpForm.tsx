import { Button, Input, Stack } from "@chakra-ui/react"
import React, { useCallback, useState } from "react"

interface TopUpFormProps {
  loading: boolean
  handleTopUp: (to: string, amount: bigint) => Promise<void>
}

const TopUpForm: React.FC<TopUpFormProps> = ({ loading, handleTopUp }) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Number(amount) * 10 ** decimals)

    handleTopUp(to, bigintAmount)
      .then(() => {
        setTo("")
        setAmount("")
      })
      .catch(e => {
        console.log(e)
      })
  }, [amount, handleTopUp, to])

  return (
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
        Top Up
      </Button>
    </Stack>
  )
}

export default TopUpForm
