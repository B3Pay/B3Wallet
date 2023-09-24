import { Table, TableCaption, Tbody, Td, Th, Thead, Tr } from "@chakra-ui/react"
import { LogEntry } from "declarations/b3_wallet/b3_wallet.did"
import { nanoToHumanReadable } from "helpers/utiles"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"

interface LogTableProps {
  logs: Array<LogEntry>
  actor?: B3Wallet
}

const LogTable: React.FC<LogTableProps> = ({ actor }) => {
  const [logs, setLogs] = useState<Array<LogEntry>>()

  useEffect(() => {
    if (!actor) {
      return
    }

    actor
      .print_log_entries()
      .then(logs => {
        setLogs(logs)
      })
      .catch(err => {
        console.error(err)
      })
  }, [])

  return (
    <Table variant="simple">
      <TableCaption>Backend Logs</TableCaption>
      <Thead>
        <Tr>
          <Th>Counter</Th>
          <Th>File</Th>
          <Th>Line</Th>
          <Th>Cycle</Th>
          <Th>Version</Th>
          <Th>Message</Th>
          <Th>Timestamp</Th>
        </Tr>
      </Thead>
      <Tbody>
        {logs?.map((log, index) => (
          <Tr key={index}>
            <Td>{log.counter.toString()}</Td>
            <Td>{log.file}</Td>
            <Td>{log.line}</Td>
            <Td>{log.cycle[0].toString()}</Td>
            <Td>{log.version}</Td>
            <Td>{log.message}</Td>
            <Td>{nanoToHumanReadable(log.timestamp)}</Td>
          </Tr>
        ))}
      </Tbody>
    </Table>
  )
}

export default LogTable
