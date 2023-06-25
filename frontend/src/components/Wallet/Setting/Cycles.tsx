import { RepeatIcon } from "@chakra-ui/icons"
import {
  CardHeader,
  IconButton,
  Progress,
  Stack,
  StackProps,
  Text
} from "@chakra-ui/react"
import { convertBigIntToNumber } from "helpers/utiles"
import React, { useEffect, useMemo, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"

const MILION_CYCLES = 1_000_000n
const TERILION_CYCLES = 1_000_000_000_000n

interface CyclesProps extends StackProps {
  actor?: B3Wallet | B3BasicWallet
  balance?: bigint
}

const Cycles: React.FC<CyclesProps> = ({
  actor,
  balance = BigInt(0),
  ...rest
}) => {
  const [balanceLoading, setBalanceLoading] = useState(false)
  const [cycleBalance, setCycleBalance] = useState(balance)

  const getBalance = async () => {
    if (!actor) {
      return
    }

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
    getBalance()
  }, [])

  const { statusColor, percent } = useMemo(() => {
    let cyclePercent = (cycleBalance * 100n) / TERILION_CYCLES

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
      spacing={0}
      {...rest}
    >
      <CardHeader m={4} p={0}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontWeight="bold">Cycle Balance</Text>
          <Stack fontWeight="semibold">
            {balanceLoading ? (
              <Text>Loading...</Text>
            ) : (
              <Stack direction="row" align="center">
                <Text>{(cycleBalance / MILION_CYCLES).toLocaleString()} M</Text>
                {actor && (
                  <IconButton
                    aria-label="Refresh"
                    icon={<RepeatIcon />}
                    onClick={getBalance}
                    size="xs"
                  />
                )}
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
