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

import { ProcessedOperation } from "declarations/b3_wallet/b3_wallet.did"
import { useMemo } from "react"
import Parent from "../Recursive"

interface ProcessedItemRequestProps extends ProcessedOperation {
  isExpanded: boolean
}

const ProcessedItem: React.FC<ProcessedItemRequestProps> = ({
  operation,
  method,
  result,
  timestamp,
  isExpanded,
  status,
  error
}) => {
  console.log(status)
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
              {stt}
            </Box>
          )}
          <Box flex="8" textAlign="left">
            {method}
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
              {operation.responses.map(([signer, response], i) => {
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
          <strong>Reason:</strong> {operation.consent_message.reason}
        </Text>
        <Text>
          <strong>Request:</strong> {operation.consent_message.title}
        </Text>
        <Text>
          <strong>Details:</strong>{" "}
          {result &&
            Object.entries(result).map(([key, value]) => (
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
        </Text>
        {stt !== "Success" &&
          Object.entries(operation).map(([key, value]) => (
            <Parent key={key} parent={key} child={value} />
          ))}
        {error.length > 0 && (
          <Stack>
            <Text color="red.500">
              <strong>Error:</strong> {error}
            </Text>
          </Stack>
        )}
      </AccordionPanel>
    </Box>
  )
}

export default ProcessedItem
