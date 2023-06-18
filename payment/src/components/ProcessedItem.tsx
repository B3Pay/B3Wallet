import {
  Box,
  Stack,
  Table,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_payment/b3_payment.did"
import { useMemo } from "react"
import Parent from "./Recursive"

interface ProcessedItemRequestProps extends ProcessedRequest {}

const ProcessedItem: React.FC<ProcessedItemRequestProps> = ({
  request,
  result,
  timestamp,
  status,
  error
}) => {
  const date = useMemo(() => {
    const time = timestamp / BigInt(1e6)
    return new Date(Number(time))
  }, [timestamp])

  const stt = Object.keys(status)[0]
  return (
    <Box overflow="hidden" p={2}>
      <Stack flex="12" textAlign="left" direction="row" justify="space-between">
        {stt === "Success" ? (
          <Box flex="1" textAlign="left" color="green.500">
            Executed
          </Box>
        ) : (
          <Box flex="1" textAlign="left" color="red.500">
            {stt}
          </Box>
        )}
        <Box flex="6" textAlign="left">
          {request.consent_message.title}
        </Box>
        <Box flex="5" textAlign="left" fontSize="xs">
          {date.toLocaleDateString()} {date.toLocaleTimeString()}
        </Box>
      </Stack>
      <Stack pb={4}>
        <Table size="sm">
          <Thead>
            <Tr>
              <Th>Signer ID</Th>
              <Th>Response</Th>
            </Tr>
          </Thead>
          <Tbody>
            {request.responses.map(([signer, response], i) => {
              const stt = Object.keys(response)[0]

              return (
                <Tr key={i} pl={2}>
                  <Td>{signer.toString()}</Td>
                  <Td
                    color={stt === "Confirm" ? "green.500" : "red.500"}
                    fontWeight="bold"
                  >
                    {Object.keys(response)[0]}
                  </Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <Text>
          <strong>Reason:</strong> {request.consent_message.reason}
        </Text>
        <Text>
          <strong>Role:</strong> {Object.keys(request.role)[0]}
        </Text>
        {result[0] &&
          Object.entries(result[0]).map(([key, value]) => (
            <Stack key={key}>
              <Parent
                parent={key}
                child={!Array.isArray(value) ? value : null}
              />
              {Array.isArray(value)
                ? value.map(value =>
                    Object.entries(value).map(([key, value]) => (
                      <Parent key={key} parent={key} child={value} />
                    ))
                  )
                : null}
            </Stack>
          ))}
        {stt !== "Success" &&
          Object.entries(request.request).map(([key, value]) => (
            <Parent key={key} parent={key} child={value} />
          ))}
        {error.length > 0 && (
          <Stack>
            <Text color="red.500">
              <strong>Error:</strong> {error}
            </Text>
          </Stack>
        )}
      </Stack>
    </Box>
  )
}

export default ProcessedItem
