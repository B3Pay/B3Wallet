import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  CardBody,
  Link,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { B3_SYSTEM_CANISTER_ID, IS_LOCAL } from "helpers/config"
import { useState } from "react"
import { B3Wallet } from "service"
import Address from "../Address"

interface DangerZoneProps {
  actor: B3Wallet
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
      window.location.reload()
    }
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
      borderColor="red.200"
    >
      {!!loadingTitle && <Loading title={loadingTitle} />}
      <Accordion allowToggle>
        <AccordionItem border="none" _focus={{ boxShadow: "none" }}>
          <Box>
            <Stack
              direction="row"
              justify="space-between"
              align="center"
              px={4}
              py={2}
            >
              <Text flex={6} fontSize="md" fontWeight="bold">
                Danger Zone
              </Text>
              <Stack fontSize="sm" fontWeight="semibold">
                <AccordionButton borderRadius="lg" flex={1}>
                  <AccordionIcon />
                </AccordionButton>
              </Stack>
            </Stack>
            <AccordionPanel>
              <CardBody
                borderTop="1px"
                borderColor="gray.200"
                position="relative"
                py={4}
                px={2}
              >
                <Box color="red.500" pb={3}>
                  For uninstalling wallet, please make sure have your{" "}
                  <Link
                    color="blue.500"
                    isExternal
                    target="_blank"
                    rel="noopener noreferrer"
                    href={
                      IS_LOCAL
                        ? "http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/"
                        : "https://nns.ic0.app/"
                    }
                  >
                    NNS Dapp
                  </Link>{" "}
                  Principal or System Principal{" "}
                  <Address
                    address={B3_SYSTEM_CANISTER_ID}
                    display="inline-flex"
                    color="blue.500"
                  />
                  as controller, or atleast have another way to reinstall code
                  to your canister.
                </Box>
                <Stack direction="row" spacing={2}>
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
            </AccordionPanel>
          </Box>
        </AccordionItem>
      </Accordion>
    </Stack>
  )
}

export default DangerZone
