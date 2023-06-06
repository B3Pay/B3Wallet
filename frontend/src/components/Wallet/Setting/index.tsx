import { Button, Stack, Text } from "@chakra-ui/react"
import { B3Wallet } from "service/actor"
import AddSigner from "./AddSigner"
import RestoreAccount from "./RestoreAccount"
import Status from "./Status"
import Wasm from "./Wasm"

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
