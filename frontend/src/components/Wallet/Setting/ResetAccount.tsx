import {
  Button,
  CardBody,
  CardHeader,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { useState } from "react"
import { B3Wallet } from "service/actor"

interface ResetAccountProps {
  actor: B3Wallet
  fetchAccounts: () => void
}

const ResetAccount: React.FC<ResetAccountProps> = ({
  fetchAccounts,
  actor
}) => {
  const [loading, setLoading] = useState(false)

  const toast = useToast()

  const resetAccountHandler = async () => {
    setLoading(true)
    const result = await actor.reset_wallet()

    toast({
      title: "Account Reset",
      description: result,
      status: "success",
      duration: 5000,
      isClosable: true
    })

    fetchAccounts()

    setLoading(false)
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      mt={4}
      position="relative"
    >
      {loading && <Loading title="Resetting Account" />}
      <CardHeader pb={2}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontSize="md" fontWeight="bold">
            Danger Zone
          </Text>
        </Stack>
      </CardHeader>
      <CardBody borderTop="1px" borderColor="gray.200">
        <Stack fontSize="sm" fontWeight="semibold">
          <Text color="gray.500">
            Reset your account to the initial state. This will remove all your
            accounts and add the default account.
          </Text>
          <Button
            colorScheme="red"
            isLoading={loading}
            onClick={resetAccountHandler}
          >
            Reset Account
          </Button>
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default ResetAccount
