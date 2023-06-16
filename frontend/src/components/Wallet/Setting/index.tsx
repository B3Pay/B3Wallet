import {
  Button,
  CardBody,
  CardHeader,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import { WalletSetting } from "declarations/b3_wallet/b3_wallet.did"
import { B3System, B3Wallet } from "service/actor"
import AddSigner from "./AddSigner"
import Controllers from "./Controllers"
import Cycles from "./Cycles"
import RestoreAccount from "./RestoreAccount"
import Status from "./Status"
import Wasm from "./Wasm"

interface SettingsProps {
  refreshWallet: () => void
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
  setting: WalletSetting
  actor: B3Wallet
  systemActor: B3System
}

const Settings: React.FC<SettingsProps> = ({
  setLoading,
  actor,
  setting,
  systemActor,
  fetchAccounts,
  refreshWallet
}) => {
  const toast = useToast()

  const resetAccount = async () => {
    if (!actor) {
      return
    }

    setLoading(true)

    const result = await actor.reset_wallet()

    toast({
      title: "Account Reset",
      description: result,
      status: "success",
      duration: 5000,
      isClosable: true
    })

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
      <Controllers actor={actor} {...setting} />
      <Wasm
        refreshWallet={refreshWallet}
        systemActor={systemActor}
        actor={actor}
        setLoading={setLoading}
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
