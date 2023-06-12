import { Button, Input, Stack } from "@chakra-ui/react"
import { BtcNetwork } from "declarations/b3_wallet/b3_wallet.did"
import React, { useCallback, useState } from "react"

interface TransferFormProps {
  network: BtcNetwork
  loading: boolean
  title: string
  handleSwap: (network: BtcNetwork, amount: bigint) => Promise<void>
}

const SwapForm: React.FC<TransferFormProps> = ({
  network,
  loading,
  title,
  handleSwap
}) => {
  const [amount, setAmount] = useState<string>("")

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Number(amount) * 10 ** decimals)

    handleSwap(network, bigintAmount)
      .then(() => {
        setAmount("")
      })
      .catch(e => {
        console.log(e)
      })
  }, [network, amount, handleSwap])

  return (
    <Stack direction="row" justify="space-between" align="center">
      <Input
        style={{
          flex: 5
        }}
        id="amount"
        alt="Amount"
        placeholder="Amount"
        type="text"
        value={amount}
        onChange={e => setAmount(e.target.value)}
      />
      <Button
        style={{
          flex: 5
        }}
        onClick={transferHandler}
        isLoading={loading}
      >
        {title}
      </Button>
    </Stack>
  )
}

export default SwapForm
