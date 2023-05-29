import {
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Text
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useMemo } from "react"

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
          <Box flex="1" textAlign="left">
            Request ID: {request.id.toString()}
          </Box>
          <Box flex="1" textAlign="left">
            {date.toLocaleDateString()} {date.toLocaleTimeString()}
          </Box>
          <AccordionIcon />
        </AccordionButton>
      </h2>
      <AccordionPanel pb={4}>
        <Text>
          <strong>Message:</strong> {consentMessage}
        </Text>
        <Text>
          <strong>Role:</strong> {Object.keys(request.role)[0]}
        </Text>
        <Text>
          <strong>Status:</strong> {stt}
        </Text>
      </AccordionPanel>
    </AccordionItem>
  )
}

export default Processed
