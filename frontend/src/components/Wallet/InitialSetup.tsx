import { Stack, Text } from "@chakra-ui/react"
import {
  WalletInititializeArgs,
  WalletSettingsAndSigners
} from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3Wallet } from "service"
import PrincipalCard from "./PrincipalCard"
import Controllers, { ControllerMap } from "./Setting/Controllers"
import Signers from "./Setting/Signers"

interface InitialSetupProps extends WalletSettingsAndSigners {
  actor: B3Wallet
  principal: string
  fetchSettingsAndSigners: () => Promise<void>
  fetchAccounts: () => Promise<void>
}

const InitialSetup: React.FC<InitialSetupProps> = ({
  actor,
  principal,
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

    const args: WalletInititializeArgs = {
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
      <PrincipalCard address={principal} />
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
            actor={actor as B3Wallet}
            isInitialPage
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
          isInitialPage
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
