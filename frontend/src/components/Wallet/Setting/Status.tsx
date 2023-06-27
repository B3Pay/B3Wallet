import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  CardBody,
  Stack,
  Stat,
  StatHelpText,
  StatLabel,
  Text
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { WalletCanisterStatus } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"
import Recursive from "../../Recursive"

interface ResponseProps {
  actor: B3Wallet | B3BasicWallet
}

const Status: React.FC<ResponseProps> = ({ actor }) => {
  const [loading, setLoading] = useState(false)
  const [status, setStatus] = useState<WalletCanisterStatus>()
  const errorToast = useToastMessage()

  const fetchStatus = async () => {
    setStatus(undefined)
    setLoading(true)

    actor
      .status()
      .then(setStatus)
      .catch(e => {
        errorToast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
        setLoading(false)
      })
      .finally(() => setLoading(false))
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
    >
      {!!loading && <Loading />}
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
                Status
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
                <Stack>
                  {status && (
                    <Accordion bg="gray.100" allowToggle>
                      <AccordionItem>
                        <Text>
                          <AccordionButton>
                            <Box as="span" flex="1" textAlign="left">
                              Wallet Status
                            </Box>
                            <AccordionIcon />
                          </AccordionButton>
                        </Text>
                        <AccordionPanel
                          pb={4}
                          overflowWrap="anywhere"
                          overflowY="scroll"
                          maxH={400}
                        >
                          <Stat>
                            <StatLabel>canister_id: &nbsp;</StatLabel>
                            <StatHelpText>
                              {status.canister_id.toString()}
                            </StatHelpText>
                          </Stat>
                          {Object.entries(status.account_status).map(
                            ([key, value]) => (
                              <Recursive key={key} parent={key} child={value} />
                            )
                          )}
                          <Stat>
                            <StatLabel>version: &nbsp;</StatLabel>
                            <StatHelpText>{status.version}</StatHelpText>
                          </Stat>
                        </AccordionPanel>
                      </AccordionItem>
                      <AccordionItem>
                        <Text>
                          <AccordionButton>
                            <Box as="span" flex="1" textAlign="left">
                              Canister Status
                            </Box>
                            <AccordionIcon />
                          </AccordionButton>
                        </Text>
                        <AccordionPanel
                          pb={4}
                          overflowWrap="anywhere"
                          overflowY="scroll"
                          maxH={400}
                        >
                          {Object.entries(status.canister_status).map(
                            ([key, value]) => (
                              <Recursive key={key} parent={key} child={value} />
                            )
                          )}
                        </AccordionPanel>
                      </AccordionItem>
                    </Accordion>
                  )}
                  <Button
                    onClick={fetchStatus}
                    isLoading={loading}
                    w="100%"
                    colorScheme="orange"
                  >
                    Fetch Status
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

export default Status
