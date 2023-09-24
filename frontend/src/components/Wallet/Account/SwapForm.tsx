import { Button, Input, Stack } from "@chakra-ui/react"
import { BtcNetwork } from "declarations/b3_wallet/b3_wallet.did"
import React, { useCallback, useState } from "react"

interface SwapFormProps {
  network: BtcNetwork
  title: string
  toAddress?: string
  noAddressInput?: boolean
  handleSwap: (network: BtcNetwork, to: string, amount: bigint) => Promise<void>
}

const SwapForm: React.FC<SwapFormProps> = ({
  network,
  title,
  toAddress,
  noAddressInput,
  handleSwap
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [loading, setLoading] = useState(false)

  const transferHandler = useCallback(async () => {
    const decimals = 8

    const bigintAmount = BigInt(Math.floor(Number(amount) * 10 ** decimals))

    setLoading(true)

    handleSwap(network, to, bigintAmount)
      .then(() => {
        setAmount("")
      })
      .catch(e => {
        console.log(e)
      })
      .finally(() => {
        setLoading(false)
      })
  }, [network, amount, handleSwap])

  return (
    <Stack direction="row" justify="space-between" align="center">
      {!noAddressInput && (
        <Input
          flex={4}
          alt="To"
          placeholder="To"
          type="text"
          value={toAddress || to}
          disabled={!!toAddress}
          onChange={e => setTo(e.target.value)}
        />
      )}
      <Input
        flex={4}
        alt="Amount"
        placeholder="Amount"
        type="text"
        value={amount}
        onChange={e => setAmount(e.target.value)}
      />
      <Button
        flex={noAddressInput ? 8 : 3}
        onClick={transferHandler}
        isLoading={loading}
      >
        {title}
      </Button>
    </Stack>
  )
}

export default SwapForm
