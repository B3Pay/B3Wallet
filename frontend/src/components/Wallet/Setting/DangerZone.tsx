import {
  Button,
  CardBody,
  CardHeader,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { B3_SYSTEM_CANISTER_ID } from "helpers/config"
import { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"
import Address from "../Address"

interface DangerZoneProps {
  actor: B3Wallet | B3BasicWallet
  fetchAccounts: () => void
  allowUninstall?: boolean
}

const DangerZone: React.FC<DangerZoneProps> = ({
  fetchAccounts,
  allowUninstall,
  actor
}) => {
  const [loadingTitle, setLoadingTitle] = useState("")

  const toast = useToast()

  const resetAccountHandler = async () => {
    setLoadingTitle("Resetting Account")
    const result = await actor.reset_accounts()

    toast({
      title: "Account Reset",
      description: result,
      status: "success",
      duration: 5000,
      isClosable: true
    })

    fetchAccounts()

    setLoadingTitle("")
  }

  const uninstallWallet = async () => {
    setLoadingTitle("Uninstalling Wallet")
    try {
      await actor.uninstall_wallet()
    } catch (error) {
      toast({
        title: "Wallet Uninstalled",
        description: error,
        status: "success",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoadingTitle("")
      setTimeout(() => {
        window.location.reload()
      }, 5000)
    }
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      mt={4}
      position="relative"
    >
      {!!loadingTitle && <Loading title={loadingTitle} />}
      <CardHeader pb={2}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontSize="md" fontWeight="bold">
            Danger Zone
          </Text>
        </Stack>
      </CardHeader>
      <CardBody borderTop="1px" borderColor="gray.200">
        <Stack direction="row" spacing={2}>
          <Text fontSize="sm">
            Reset your account or uninstall your wallet.
          </Text>
          <Text fontSize="sm" color="red.500">
            This action is irreversible.
          </Text>
          <Text fontSize="sm" color="red.500">
            For uninstalling wallet, please make sure you have NNS pricipal or
            System Principal&apos;s (
            <Address address={B3_SYSTEM_CANISTER_ID} />) as controller, or
            atleast have another way to reinstall code to your canister.
          </Text>
          <Button
            flex={1}
            colorScheme="red"
            isLoading={!!loadingTitle}
            onClick={uninstallWallet}
            isDisabled={!allowUninstall}
          >
            Uninstall Wallet
          </Button>
          <Button
            flex={1}
            colorScheme="orange"
            isLoading={!!loadingTitle}
            onClick={resetAccountHandler}
          >
            Reset Account
          </Button>
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default DangerZone
