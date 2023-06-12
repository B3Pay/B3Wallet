import { RepeatIcon } from "@chakra-ui/icons"
import { CardHeader, IconButton, Progress, Stack, Text } from "@chakra-ui/react"
import { convertBigIntToNumber } from "helpers/utiles"
import React, { useEffect, useMemo, useState } from "react"
import { B3Wallet } from "service/actor"

const MILION_CYCLES = 1_000_000n
const TERILION_CYCLES = 1_000_000_000_000n

interface CyclesProps {
  actor: B3Wallet
}

const Cycles: React.FC<CyclesProps> = ({ actor }) => {
  const [balanceLoading, setBalanceLoading] = useState(false)
  const [cycleBalance, setCycleBalance] = useState(BigInt(0))

  const getBalance = async () => {
    setBalanceLoading(true)
    actor
      .canister_cycle_balance()
      .then(balance => {
        setCycleBalance(balance)
        setBalanceLoading(false)
      })
      .catch(err => {
        console.error(err)
        setBalanceLoading(false)
      })
  }

  useEffect(() => {
    if (!actor) {
      return
    }

    getBalance()
  }, [])

  const { statusColor, percent } = useMemo(() => {
    let cyclePercent = (cycleBalance / TERILION_CYCLES) * 100n

    let percent = convertBigIntToNumber(cyclePercent)

    if (percent < 10) {
      return {
        statusColor: "red",
        percent
      }
    }

    if (percent < 50) {
      return {
        statusColor: "yellow",
        percent
      }
    }

    return {
      statusColor: "green",
      percent
    }
  }, [cycleBalance])
  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
    >
      <CardHeader pb={2}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontSize="md" fontWeight="bold">
            Cycle Balance
          </Text>
          <Stack fontSize="sm" fontWeight="semibold">
            {balanceLoading ? (
              <Text>Loading...</Text>
            ) : (
              <Stack direction="row" align="center">
                <Text>{(cycleBalance / MILION_CYCLES).toLocaleString()} M</Text>
                <IconButton
                  aria-label="Refresh"
                  icon={<RepeatIcon />}
                  onClick={getBalance}
                  size="xs"
                />
              </Stack>
            )}
          </Stack>
        </Stack>
      </CardHeader>
      <Progress
        hasStripe
        value={percent}
        colorScheme={statusColor}
        isAnimated={balanceLoading}
      />
    </Stack>
  )
}

export default Cycles
