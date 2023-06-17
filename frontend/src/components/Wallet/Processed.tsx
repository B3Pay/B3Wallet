import {
  AccordionButton,
  AccordionIcon,
  AccordionPanel,
  Box,
  Stack,
  Text
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useMemo } from "react"
import Parent from "../Recursive"

interface ProcessedRequestProps extends ProcessedRequest {
  isExpanded: boolean
}

const Processed: React.FC<ProcessedRequestProps> = ({
  request,
  result,
  timestamp,
  isExpanded,
  status
}) => {
  const date = useMemo(() => {
    const time = timestamp / BigInt(1e6)
    return new Date(Number(time))
  }, [timestamp])

  console.log({ result })

  const stt = Object.keys(status)[0]
  return (
    <Box
      borderWidth="1px"
      borderRadius={isExpanded ? "lg" : "none"}
      overflow="hidden"
      my={isExpanded ? 2 : 0}
    >
      <AccordionButton>
        <Stack
          flex="12"
          textAlign="left"
          direction="row"
          justify="space-between"
        >
          {stt === "Success" ? (
            <Box flex="1" textAlign="left" color="green.500">
              Executed
            </Box>
          ) : (
            <Box flex="1" textAlign="left" color="red.500">
              Failed
            </Box>
          )}
          <Box flex="8" textAlign="left">
            {request.consent_message.title}
          </Box>
          <Box flex="4" textAlign="left">
            {date.toLocaleDateString()} {date.toLocaleTimeString()}
          </Box>
        </Stack>
        <AccordionIcon />
      </AccordionButton>
      <AccordionPanel pb={4}>
        <Text>
          <strong>Message:</strong> {request.consent_message.message}
        </Text>
        <Text>
          <strong>Role:</strong> {Object.keys(request.role)[0]}
        </Text>
        <strong>Result:</strong>
        {Object.entries(result[0]).map(([key, value]) => (
          <Stack>
            <Parent key={key} parent={key} child={null} />
            {value.map(value =>
              Object.entries(value).map(([key, value]) => (
                <Parent key={key} parent={key} child={value} />
              ))
            )}
          </Stack>
        ))}
      </AccordionPanel>
    </Box>
  )
}

export default Processed
