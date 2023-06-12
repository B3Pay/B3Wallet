import { Button, CardBody, CardHeader, Stack, Text } from "@chakra-ui/react"
import { B3Wallet } from "service/actor"
import AddSigner from "./AddSigner"
import Cycles from "./Cycles"
import RestoreAccount from "./RestoreAccount"
import Status from "./Status"
import Wasm from "./Wasm"

interface SettingsProps {
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
  actor: B3Wallet
}

const Settings: React.FC<SettingsProps> = ({
  setLoading,
  actor,
  fetchAccounts
}) => {
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

  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Settings
      </Text>
      <Cycles actor={actor} />
      <AddSigner actor={actor} />
      <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
      <Status actor={actor} />
      <Wasm
        actor={actor}
        setLoading={setLoading}
        fetchAccounts={fetchAccounts}
      />
      <Stack
        direction="column"
        borderWidth="1px"
        borderRadius="lg"
        overflow="hidden"
      >
        <CardHeader pb={2}>
          <Stack direction="row" justify="space-between" align="center">
            <Text fontSize="md" fontWeight="bold">
              Danger Zone
            </Text>
          </Stack>
        </CardHeader>
        <CardBody borderTop="1px" borderColor="gray.200">
          <Stack fontSize="sm" fontWeight="semibold">
            <Text color="gray.500">
              Reset your account to the initial state. This will remove all your
              accounts and add the default account.
            </Text>
            <Button colorScheme="red" onClick={resetAccount}>
              Reset Account
            </Button>
          </Stack>
        </CardBody>
      </Stack>
    </Stack>
  )
}

export default Settings
