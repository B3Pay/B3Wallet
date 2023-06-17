import { Box, BoxProps } from "@chakra-ui/react"
import React from "react"
import LoadingDots from "../LoadingDots"

interface BalanceProps extends BoxProps {
  amount: bigint
  symbol: string
  loading?: boolean
}

const Balance: React.FC<BalanceProps> = ({
  amount,
  symbol,
  loading,
  ...rest
}) => {
  if (!amount) {
    return <Box {...rest}>{loading ? <LoadingDots /> : `0.0 ${symbol}`}</Box>
  }

  const decimals = 8
  const divisor = BigInt(10 ** decimals)
  const whole = Number(amount / divisor)
  const fraction = String(amount % divisor)
    .padStart(decimals, "0")
    .slice(0, 8)

  return (
    <Box {...rest}>
      {loading ? <LoadingDots /> : `${whole}.${fraction} ${symbol}`}
    </Box>
  )
}

export default Balance
