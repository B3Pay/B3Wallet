import { Button, Input, Stack } from "@chakra-ui/react"
import React, { useCallback, useState } from "react"

interface TopUpFormProps {
  address: string
  loading: boolean
  title: string
  handleTransfer: (from: string, to: string, amount: bigint) => Promise<void>
}

const TopUpForm: React.FC<TopUpFormProps> = ({
  address,
  loading,
  title,
  handleTransfer: handleTransfer
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
        {title}
      </Button>
    </Stack>
  )
}

export default TopUpForm
