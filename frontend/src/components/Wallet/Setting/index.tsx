import { Alert, AlertIcon, Stack, Text } from "@chakra-ui/react"
import { WalletSettings } from "declarations/b3_wallet/b3_wallet.did"
import { B3_SYSTEM_CANISTER_ID } from "helpers/config"
import { B3System, B3Wallet } from "service"
import Address from "../Address"
import PrincipalCard from "../PrincipalCard"
import Controllers from "./Controllers"
import Cycles from "./Cycles"
import DangerZone from "./DangerZone"
import RestoreAccount from "./RestoreAccount"
import Signers, { UserMap } from "./Signers"
import Status from "./Status"
import Wasm from "./Wasm"

interface SettingsProps {
  refreshWallet: () => void
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
  signers: UserMap
  principal: string
  settings: WalletSettings
  actor: B3Wallet
  systemActor: B3System
}

const Settings: React.FC<SettingsProps> = ({
  actor,
  settings,
  signers,
  setLoading,
  principal,
  systemActor,
  fetchAccounts,
  refreshWallet
}) => {
  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Settings
      </Text>
      <PrincipalCard address={principal} />
      <Alert status="warning" borderRadius="base">
        <AlertIcon />
        <Text fontSize="sm" as="div">
          If you have trouble with your wallet canister, you should try to
          update it.
          <br />
          If update fails, you can uninstall from below and reinstall it. don't
          forget to add system
          <Address
            address={B3_SYSTEM_CANISTER_ID}
            display="inline-flex"
            color="blue.500"
          />
          canister id as a controller before uninstalling.
          <br />
          Note: You need more that 100 million cycles to update your wallet
          canister.
        </Text>
      </Alert>
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
      <Status actor={actor} />
      <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
      <DangerZone
        actor={actor}
        fetchAccounts={fetchAccounts}
        allowUninstall={settings?.controllers.length > 2}
      />
    </Stack>
  )
}

export default Settings
