import {
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Stack,
  Text
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useMemo } from "react"
import Parent from "../Recursive"

interface ProcessedRequestProps extends ProcessedRequest {}

const Processed: React.FC<ProcessedRequestProps> = ({
  request,
  message,
  timestamp,
  status
}) => {
  const consentMessage = useMemo(() => {
    if ("Valid" in message) return message.Valid.consent_message
    if ("MalformedCall" in message) return message.MalformedCall.description
    if ("Forbidden" in message) return message.Forbidden.description
    if ("Other" in message) return message.Other
  }, [message])

  const date = useMemo(() => {
    const time = timestamp / BigInt(1e6)
    return new Date(Number(time))
  }, [timestamp])

  const stt = Object.keys(status)[0]
  return (
    <AccordionItem
      bgColor={stt === "Success" ? "green.100" : "red.100"}
      border="none"
      _focus={{ boxShadow: "none" }}
    >
      <h2>
        <AccordionButton>
          <Stack
            flex="12"
            textAlign="left"
            direction="row"
            justify="space-between"
          >
            <Box flex="8" textAlign="left">
              {request.consent_message.method}
            </Box>
            <Box flex="4" textAlign="left">
              {date.toLocaleDateString()} {date.toLocaleTimeString()}
            </Box>
          </Stack>
          <AccordionIcon />
        </AccordionButton>
      </h2>
      <AccordionPanel pb={4}>
        <Text>
          <strong>Status:</strong> {stt}
        </Text>
        <Text>
          <strong>Message:</strong> {consentMessage}
        </Text>
        <Text>
          <strong>Role:</strong> {Object.keys(request.role)[0]}
        </Text>
        <strong>Args:</strong>
        {Object.entries(request.consent_message.arg).map(([key, value]) => (
          <Parent key={key} parent={key} child={value} />
        ))}
      </AccordionPanel>
    </AccordionItem>
  )
}

export default Processed
