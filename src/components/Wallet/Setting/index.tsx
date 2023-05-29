import { Stack, Text } from "@chakra-ui/react"
import { B3Wallet } from "service/actor"
import RestoreAccount from "./RestoreAccount"
import Signer from "./Signer"
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
  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Settings
      </Text>
      <Signer actor={actor} />
      <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
      <Status actor={actor} />
      <Wasm actor={actor} version={version} setLoading={setLoading} />
    </Stack>
  )
}

export default Settings
