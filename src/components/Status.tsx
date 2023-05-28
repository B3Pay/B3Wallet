import {
  Button,
  Card,
  Stack,
  Stat,
  StatHelpText,
  StatLabel
} from "@chakra-ui/react"
import { WalletCanisterStatus } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3User } from "service/actor"

interface ResponseProps {
  actor: B3User
}

const parent = (key: string, value: any) =>
  value && typeof value === "object" ? (
    <Stat key={key}>
      <StatLabel>{key}: &nbsp;</StatLabel>
      {child(value)}
    </Stat>
  ) : (
    <Stat key={key}>
      <StatLabel>{key}: &nbsp;</StatLabel>
      <StatHelpText>{value?.toString()}</StatHelpText>
    </Stat>
  )

const child = (value: any) =>
  value &&
  (value._isPrincipal ? (
    value.toText()
  ) : typeof value === "object" ? (
    Array.isArray(value) || typeof value[0] === "number" ? (
      value.toString()
    ) : (
      <Stack ml={2}>
        {Object.entries(value).map(([key, value]) => parent(key, value))}
      </Stack>
    )
  ) : (
    value.toString()
  ))

const Status: React.FC<ResponseProps> = ({ actor }) => {
  const [loading, setLoading] = useState(false)
  const [status, setStatus] = useState<WalletCanisterStatus>()

  const fetchStatus = async () => {
    setStatus(undefined)
    setLoading(true)

    actor
      .status()
      .then(setStatus)
      .catch(console.error)
      .finally(() => setLoading(false))
  }

  return (
    <Stack position="relative">
      {status && (
        <Card
          overflowWrap="anywhere"
          overflowY="scroll"
          maxH={400}
          bg="gray.100"
          p={4}
        >
          <Stack>
            {Object.entries(status).map(([key, value]) => parent(key, value))}
          </Stack>
        </Card>
      )}
      <Button onClick={fetchStatus} isLoading={loading}>
        Fetch Status
      </Button>
    </Stack>
  )
}

export default Status
