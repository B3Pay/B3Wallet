import {
  Alert,
  AlertIcon,
  Button,
  Input,
  Stack,
  Table,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { LogEntry } from "declarations/b3_wallet/b3_wallet.did"
import { B3_SYSTEM_CANISTER_ID } from "helpers/config"
import { nanoToHumanReadable } from "helpers/utiles"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"

interface LogProps {
  actor?: B3Wallet
  loading?: boolean
  setLoading?: React.Dispatch<React.SetStateAction<boolean>>
}

const Logs: React.FC<LogProps> = ({ actor, loading, setLoading }) => {
  const [logs, setLogs] = useState<Array<LogEntry>>()
  const [description, setDescription] = useState("")

  const fetchLogs = async () => {
    if (!actor) {
      return
    }

    setLoading(true)

    actor
      .print_log_entries()
      .then(logs => {
        setLogs(logs)
        setLoading(false)
      })
      .catch(err => {
        console.error(err)
        setLoading(false)
      })
  }

  useEffect(() => {
    fetchLogs()
  }, [])

  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold" textAlign="center">
        Logs
      </Text>
      <Stack borderWidth="1px" borderRadius="lg" overflow="hidden" padding={4}>
        <Text fontSize="xl" fontWeight="bold">
          Report Bug
        </Text>
        <Alert status="warning" borderRadius="base">
          <AlertIcon />
          <Text fontSize="sm" as="div">
            If you have trouble with your wallet canister, please report it to
            us. We will try to fix it as soon as possible.
            <br />
            <strong>Note: </strong>
            This will send your 10 latest logs to us.
          </Text>
        </Alert>
        <Stack direction="row" align="center">
          <Input
            flex={8}
            type="text"
            placeholder="Description"
            onChange={e => setDescription(e.target.value)}
            value={description}
          />
          <Button
            flex={4}
            isLoading={loading}
            loadingText="Reporting"
            isDisabled={description.length < 5}
            colorScheme="blue"
            onClick={async () => {
              if (!actor) {
                return
              }
              try {
                setLoading(true)
                const principal = Principal.fromText(B3_SYSTEM_CANISTER_ID)

                if (!principal) {
                  return
                }

                await actor.report_bug(principal, description)
                fetchLogs()
              } catch (e) {
                setLoading(false)
                console.error(e)
              }
            }}
          >
            report
          </Button>
        </Stack>
      </Stack>
      <Table size="sm" overflow="scroll" borderWidth="1px" borderRadius="lg">
        <Thead>
          <Tr>
            <Th>ID</Th>
            <Th>Cycle</Th>
            <Th>Message</Th>
            <Th>Timestamp</Th>
          </Tr>
        </Thead>
        <Tbody>
          {logs?.map((log, index) => (
            <Tr key={index}>
              <Td>{log.counter.toString()}</Td>
              <Td>{log.cycle[0].toString()}</Td>
              <Td>{log.message}</Td>
              <Td>{nanoToHumanReadable(log.timestamp)}</Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </Stack>
  )
}

export default Logs
