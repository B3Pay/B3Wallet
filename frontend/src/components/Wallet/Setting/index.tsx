import { Stack, Text } from "@chakra-ui/react"
import { AuthClient } from "@dfinity/auth-client"
import { WalletSettings } from "declarations/b3_wallet/b3_wallet.did"
import { B3BasicWallet, B3System, B3Wallet } from "service"
import Address from "../Address"
import Controllers from "./Controllers"
import Cycles from "./Cycles"
import DangerZone from "./DangerZone"
import RestoreAccount from "./RestoreAccount"
import Signers, { SignerMap } from "./Signers"
import Status from "./Status"
import Wasm from "./Wasm"

interface SettingsProps {
  refreshWallet: () => void
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
  signers: SignerMap
  authClient: AuthClient
  settings: WalletSettings
  actor: B3Wallet | B3BasicWallet
  systemActor: B3System
}

const Settings: React.FC<SettingsProps> = ({
  actor,
  settings,
  signers,
  setLoading,
  authClient,
  systemActor,
  fetchAccounts,
  refreshWallet
}) => {
  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Settings
      </Text>
      <Text fontSize="large" fontWeight="bold" mt={2}>
        Your Principal
      </Text>
      <Address
        address={authClient.getIdentity().getPrincipal().toString()}
        overflow="hidden"
        px={2}
      />
      <Cycles actor={actor} />
      {signers && (
        <Signers
          actor={actor as B3Wallet}
          refetch={refreshWallet}
          signers={signers}
        />
      )}
      <Stack position="relative" spacing={4}>
        <Controllers
          actor={actor}
          controllers={settings?.controllers}
          refetch={refreshWallet}
        />
      </Stack>
      <Wasm
        refreshWallet={refreshWallet}
        systemActor={systemActor}
        actor={actor}
        setLoading={setLoading}
      />
      <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
      <Status actor={actor} />
      <DangerZone
        actor={actor}
        fetchAccounts={fetchAccounts}
        allowUninstall={settings?.controllers.length > 2}
      />
    </Stack>
  )
}

export default Settings
