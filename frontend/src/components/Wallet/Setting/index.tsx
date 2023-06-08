import { RepeatIcon } from "@chakra-ui/icons"
import {
  Button,
  CardHeader,
  IconButton,
  Progress,
  Stack,
  Text
} from "@chakra-ui/react"
import { convertBigIntToNumber } from "helpers/utiles"
import { useEffect, useMemo, useState } from "react"
import { B3Wallet } from "service/actor"
import AddSigner from "./AddSigner"
import RestoreAccount from "./RestoreAccount"
import Status from "./Status"
import Wasm from "./Wasm"

const MILION_CYCLES = 1_000_000n
const TERILION_CYCLES = 1_000_000_000_000n

interface SettingsProps {
  version: string
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
  actor: B3Wallet
}

const Settings: React.FC<SettingsProps> = ({
  version,
  setLoading,
  actor,
  fetchAccounts
}) => {
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

  const resetAccount = async () => {
    if (!actor) {
      return
    }

    setLoading(true)

    const result = await actor.reset_wallet()

    console.log(result)

    fetchAccounts()

    setLoading(false)
  }

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
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Settings
      </Text>
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
                  <Text>
                    {(cycleBalance / MILION_CYCLES).toLocaleString()} M
                  </Text>
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
      <AddSigner actor={actor} />
      <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
      <Button colorScheme="red" onClick={resetAccount}>
        Reset Account
      </Button>
      <Status actor={actor} />
      <Wasm
        actor={actor}
        version={version}
        setLoading={setLoading}
        fetchAccounts={fetchAccounts}
      />
    </Stack>
  )
}

export default Settings
