import {
  Box,
  Button,
  CardBody,
  CloseButton,
  FormControl,
  Input,
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
import { ManagementCanisterRecord } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import Loading from "components/Loading"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { CanisterStatus } from "service"
import Address from "../Address"
import Cycles from "./Cycles"

interface ControllersProps extends CanisterStatus {
  actor: ManagementCanisterRecord
  canisterId: string
  setParentControllers: (controller: Array<Principal>) => void
}

const CanisterControllers: React.FC<ControllersProps> = ({
  actor,
  cycles,
  settings,
  setParentControllers,
  canisterId
}) => {
  const [controllers, setControllers] = useState<Principal[]>(
    settings.controllers
  )

  const [loading, setLoading] = useState(false)

  const [principal, setPrincipal] = useState("")

  const errorToast = useToastMessage()

  const removeController = (index: number) => {
    setControllers(prev => {
      const newControllers = [...prev]
      newControllers.splice(index, 1)
      return newControllers
    })
  }

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    let signerId: Principal

    try {
      signerId = Principal.fromText(principal)
    } catch (e) {
      return errorToast({
        title: "Invalid Principal",
        description: e.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    }

    setControllers(prev => [signerId, ...prev])
    setPrincipal("")
  }

  const edited =
    JSON.stringify(settings.controllers) !== JSON.stringify(controllers)

  const handleUpdateController = async () => {
    setLoading(true)

    let canister_id: Principal

    try {
      canister_id = Principal.fromText(canisterId)
    } catch (e) {
      return errorToast({
        title: "Invalid Principal",
        description: e.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    }

    if (!edited) {
      return setLoading(false)
    }

    if (controllers.length === 0) {
      return errorToast({
        title: "Invalid Controllers",
        description: "At least one controller is required.",
        status: "error",
        duration: 9000,
        isClosable: true
      })
    }

    try {
      await actor.update_settings({
        canister_id,
        settings: {
          controllers: [controllers],
          freezing_threshold: [],
          memory_allocation: [],
          compute_allocation: []
        }
      })

      setParentControllers(controllers)
    } catch (err) {
      console.error(err)
      errorToast({
        title: "Error",
        description: err.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <Stack pt={2}>
      <Cycles balance={cycles} />
      <Stack
        direction="column"
        borderWidth="1px"
        borderRadius="lg"
        overflow="hidden"
        position="relative"
      >
        {(!controllers || loading) && <Loading title="Loading controllers" />}
        <Box>
          <Stack
            direction="row"
            justify="space-between"
            align="center"
            px={4}
            py={2}
          >
            <Text fontSize="md" fontWeight="bold">
              Controllers
            </Text>
          </Stack>
          <CardBody borderTop="1px" borderColor="gray.200" m={0} p={0}>
            <TableContainer minH={75}>
              <Table size="sm">
                <Thead>
                  <Tr>
                    <Th>Controller ID</Th>
                    <Th></Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {controllers?.map((controller, index) => (
                    <Tr key={index}>
                      <Td>
                        <Address address={controller.toString()} noIcon />
                      </Td>
                      <Td>
                        <CloseButton
                          color="red"
                          onClick={() => removeController(index)}
                        />
                      </Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            </TableContainer>
            <Box
              as="form"
              onSubmit={handleSubmit}
              p={2}
              borderTop="1px"
              borderColor="gray.200"
            >
              <Stack
                alignItems="center"
                justify="space-between"
                direction="row"
              >
                <FormControl isRequired flex={5}>
                  <Input
                    value={principal}
                    onChange={e => setPrincipal(e.target.value)}
                    placeholder="Principal"
                  />
                </FormControl>
                <Button colorScheme="orange" type="submit" flex={3}>
                  Add
                </Button>
              </Stack>
              {edited && (
                <Stack direction="row" justify="space-between" mt={4}>
                  <Button
                    onClick={() => {
                      setControllers(settings.controllers)
                    }}
                    isLoading={loading}
                    colorScheme="red"
                    flex={4}
                  >
                    Cancel
                  </Button>
                  <Button
                    flex={8}
                    onClick={handleUpdateController}
                    isLoading={loading}
                    colorScheme="blue"
                  >
                    Update
                  </Button>
                </Stack>
              )}
            </Box>
          </CardBody>
        </Box>
      </Stack>
    </Stack>
  )
}

export default CanisterControllers
