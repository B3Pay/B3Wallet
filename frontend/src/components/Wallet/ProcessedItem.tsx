import {
  AccordionButton,
  AccordionIcon,
  AccordionPanel,
  Box,
  Stack,
  Table,
  TableContainer,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useMemo } from "react"
import Parent from "../Recursive"

interface ProcessedItemRequestProps extends ProcessedRequest {
  isExpanded: boolean
}

const ProcessedItem: React.FC<ProcessedItemRequestProps> = ({
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
              Rejected
            </Box>
          )}
          <Box flex="8" textAlign="left">
            {request.consent_message.title}
          </Box>
          <Box flex="4" textAlign="left" fontSize="sm">
            {date.toLocaleDateString()} {date.toLocaleTimeString()}
          </Box>
        </Stack>
        <AccordionIcon />
      </AccordionButton>
      <AccordionPanel pb={4}>
        <TableContainer>
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
        </TableContainer>
        <Text>
          <strong>Reason:</strong> {request.consent_message.reason}
        </Text>
        <Text>
          <strong>Role:</strong> {Object.keys(request.role)[0]}
        </Text>
        {result[0] &&
          Object.entries(result[0]).map(([key, value]) => (
            <Stack key={key}>
              <Parent parent={key} child={null} />
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

export default ProcessedItem
