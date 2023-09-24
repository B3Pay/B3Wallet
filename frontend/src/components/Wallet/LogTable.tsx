import { Table, TableCaption, Tbody, Td, Th, Thead, Tr } from "@chakra-ui/react"
import { LogEntry } from "declarations/b3_wallet/b3_wallet.did"
import { nanoToHumanReadable } from "helpers/utiles"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"

interface LogTableProps {
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
        setLogs(logs.reverse())
      })
      .catch(err => {
        console.error(err)
      })
  }, [])

  return (
    <Table size="sm">
      <TableCaption>Backend Logs</TableCaption>
      <Thead>
        <Tr>
          <Th>Counter</Th>
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
