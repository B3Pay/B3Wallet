import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  Stack,
  Stat,
  StatHelpText,
  StatLabel,
  Text
} from "@chakra-ui/react"
import { WalletCanisterStatus } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3Wallet } from "service/actor"
import Recursive from "../../Recursive"

interface ResponseProps {
  actor: B3Wallet
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
    <Stack position="relative">
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
                <StatHelpText>{status.canister_id.toString()}</StatHelpText>
              </Stat>
              {Object.entries(status.account_status).map(([key, value]) => (
                <Recursive key={key} parent={key} child={value} />
              ))}
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
              {Object.entries(status.canister_status).map(([key, value]) => (
                <Recursive key={key} parent={key} child={value} />
              ))}
            </AccordionPanel>
          </AccordionItem>
        </Accordion>
      )}
      <Button onClick={fetchStatus} isLoading={loading}>
        Fetch Status
      </Button>
    </Stack>
  )
}

export default Status
