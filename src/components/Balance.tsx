import { Box } from "@chakra-ui/react"
import React from "react"
import LoadingDots from "./LoadingDots"

interface BalanceProps {
  amount: bigint
  chain: string
  loading?: boolean
}

const Balance: React.FC<BalanceProps> = ({ amount, chain, loading }) => {
  if (!amount) {
    return <Box>{loading ? <LoadingDots /> : `0.0 ${chain}`}</Box>
  }

  const decimals = 8
  const divisor = BigInt(10 ** decimals)
  const whole = Number(amount / divisor)
  const fraction = String(amount % divisor)
    .padStart(decimals, "0")
    .slice(0, 4)

  return (
    <Box>{loading ? <LoadingDots /> : `${whole}.${fraction} ${chain}`}</Box>
  )
}

export default Balance
