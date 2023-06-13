import { Button, Input, Stack } from "@chakra-ui/react"
import { BtcNetwork } from "declarations/b3_wallet/b3_wallet.did"
import React, { useCallback, useState } from "react"

interface SwapFormProps {
  network: BtcNetwork
  loading: boolean
  title: string
  toAddress?: string
  handleSwap: (network: BtcNetwork, to: string, amount: bigint) => Promise<void>
}

const SwapForm: React.FC<SwapFormProps> = ({
  network,
  loading,
  title,
  toAddress,
  handleSwap
}) => {
  const [amount, setAmount] = useState<string>("")
  const [to, setTo] = useState<string>("")

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Number(amount) * 10 ** decimals)

    handleSwap(network, to, bigintAmount)
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
        flex={4}
        alt="To"
        placeholder="To"
        type="text"
        value={toAddress || to}
        disabled={!!toAddress}
        onChange={e => setTo(e.target.value)}
      />
      <Input
        flex={4}
        alt="Amount"
        placeholder="Amount"
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

export default SwapForm
