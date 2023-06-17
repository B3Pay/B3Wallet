import { Stack, Text } from "@chakra-ui/react"
import { AuthClient } from "@dfinity/auth-client"
import {
  InititializeWalletArgs,
  WalletSettingsAndSigners
} from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"
import Address from "./Address"
import Controllers, { ControllerMap } from "./Setting/Controllers"
import Signers from "./Setting/Signers"

interface InitialSetupProps extends WalletSettingsAndSigners {
  actor: B3Wallet | B3BasicWallet
  authClient: AuthClient
  fetchSettingsAndSigners: () => Promise<void>
  fetchAccounts: () => Promise<void>
}

const InitialSetup: React.FC<InitialSetupProps> = ({
  actor,
  authClient,
  settings,
  signers,
  fetchSettingsAndSigners,
  fetchAccounts
}) => {
  const [isInitializing, setIsInitializing] = useState(false)
  const errorToast = useToastMessage()

  const handleInitialize = async (controllers: ControllerMap) => {
    console.log("handleInitialize", controllers)
    setIsInitializing(true)

    const args: InititializeWalletArgs = {
      controllers,
      metadata: []
    }

    try {
      await actor.init_wallet(args)
    } catch (err) {
      console.error(err)
      errorToast({
        title: "Error",
        description: err.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    } finally {
      await fetchAccounts()
      await fetchSettingsAndSigners()
      setIsInitializing(false)
    }
  }

  return (
    <Stack>
      <Text
        fontSize="xl"
        fontWeight="bold"
        textAlign="center"
        borderBottom="1px"
        borderColor="gray.200"
        py={2}
      >
        Initial Setup
      </Text>
      <Text fontSize="large" fontWeight="bold" mt={2}>
        Wallet Address
      </Text>
      <Address
        address={authClient.getIdentity().getPrincipal().toString()}
        overflow="hidden"
        px={2}
      />
      {signers && (
        <Stack spacing={2} paddingTop={2}>
          <Stack spacing={2} ml={2}>
            <Text fontSize="large" fontWeight="bold">
              Add or remove signer.
            </Text>
            <Text fontSize="sm">You can add more controller later.</Text>
            <Text fontSize="small" color="gray.600">
              Note: The system canister is a connected canister by default, it
              only have access to some data information, you can remove it if
              you want.
            </Text>
          </Stack>
          <Signers
            signers={signers}
            actor={actor}
            refetch={fetchSettingsAndSigners}
            pt={2}
          />
        </Stack>
      )}
      <Stack spacing={2} paddingTop={2} position="relative">
        <Stack spacing={2} ml={2}>
          <Text fontSize="large" fontWeight="bold">
            Add or remove controller.
          </Text>
          <Text fontSize="sm">You can add more controller later.</Text>
          <Text fontSize="small" color="gray.600">
            Note: the wallet canister itself always is a controller, if you want
            to link this canister to NNS you should add principal of NNS as a
            controller.
          </Text>
        </Stack>
        <Controllers
          actor={actor}
          controllers={settings?.controllers}
          refetch={fetchSettingsAndSigners}
          handleInitialize={handleInitialize}
          isInitializing={isInitializing}
        />
      </Stack>
    </Stack>
  )
}

export default InitialSetup
