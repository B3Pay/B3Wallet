import { RepeatIcon } from "@chakra-ui/icons"
import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  CardBody,
  CloseButton,
  FormControl,
  IconButton,
  Input,
  Stack,
  StackProps,
  Table,
  TableContainer,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import Loading from "components/Loading"
import { Controller } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useEffect, useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"
import Address from "../Address"

export type ControllerMap = Array<[Principal, Controller]>

interface ControllersProps extends StackProps {
  actor: B3Wallet | B3BasicWallet
  refetch: () => void
  isInitialPage?: boolean
  controllers?: ControllerMap
  isInitializing?: boolean
  handleInitialize?: (controllers: ControllerMap) => Promise<void>
}

const Controllers: React.FC<ControllersProps> = ({
  actor,
  refetch,
  controllers,
  isInitialPage,
  isInitializing,
  handleInitialize,
  ...rest
}) => {
  const [controllerMap, setControllerMap] = useState<ControllerMap>()

  const [loading, setLoading] = useState(false)

  const [principal, setPrincipal] = useState("")
  const [name, setName] = useState("")

  const errorToast = useToastMessage()

  useEffect(() => {
    if (controllers) {
      setControllerMap(controllers)
    }
  }, [controllers])

  const removeController = (index: number, name: string) => {
    if (name === "self") {
      errorToast({
        description: "Cannot remove self.",
        status: "error",
        duration: 9000,
        isClosable: true
      })

      return
    }

    setControllerMap(prev => {
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

    const newController: Controller = {
      name,
      metadata: []
    }

    setControllerMap(prev => [...prev, [signerId, newController]])
    setPrincipal("")
    setName("")
  }

  const handleUpdateController = async () => {
    setLoading(true)

    try {
      let result = await actor.update_controller(controllerMap)

      setControllerMap(result)
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

  const handleRefreshControllers = async () => {
    setLoading(true)

    try {
      await actor.refresh_settings()

      refetch()
      setLoading(false)
    } catch (e) {
      errorToast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })

      setLoading(false)
    }
  }

  const edited = JSON.stringify(controllerMap) !== JSON.stringify(controllers)

  return (
    <Stack {...rest}>
      <Stack
        direction="column"
        borderWidth="1px"
        borderRadius="lg"
        overflow="hidden"
        position="relative"
      >
        {(!controllerMap || loading || isInitializing) && (
          <Loading title="Loading controllers" />
        )}
        <Accordion allowToggle defaultIndex={isInitialPage ? [0] : undefined}>
          <AccordionItem border="none" _focus={{ boxShadow: "none" }}>
            {({ isExpanded }) => (
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
                  <Stack fontSize="sm" fontWeight="semibold">
                    <Stack direction="row" align="center">
                      {isExpanded && (
                        <IconButton
                          aria-label="Refresh"
                          icon={<RepeatIcon />}
                          onClick={handleRefreshControllers}
                          size="xs"
                        />
                      )}
                      <AccordionButton borderRadius="lg">
                        <AccordionIcon />
                      </AccordionButton>
                    </Stack>
                  </Stack>
                </Stack>
                <AccordionPanel>
                  <CardBody borderTop="1px" borderColor="gray.200" m={0} p={0}>
                    <TableContainer minH={75}>
                      <Table size="sm">
                        <Thead>
                          <Tr>
                            <Th>Controller ID</Th>
                            <Th>Name</Th>
                            <Th></Th>
                          </Tr>
                        </Thead>
                        <Tbody>
                          {controllerMap?.map(
                            ([controller, { name }], index) => (
                              <Tr key={index}>
                                <Td>
                                  <Address
                                    address={controller.toString()}
                                    noIcon
                                  />
                                </Td>
                                <Td>{name}</Td>
                                <Td>
                                  <CloseButton
                                    color="red"
                                    onClick={() =>
                                      removeController(index, name)
                                    }
                                  />
                                </Td>
                              </Tr>
                            )
                          )}
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
                        <FormControl isRequired flex={4}>
                          <Input
                            value={name}
                            onChange={e => setName(e.target.value)}
                            placeholder="Name"
                          />
                        </FormControl>
                        <Button colorScheme="orange" type="submit" flex={3}>
                          Add
                        </Button>
                      </Stack>
                      {!isInitialPage && edited && (
                        <Stack direction="row" justify="space-between" mt={4}>
                          <Button
                            onClick={refetch}
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
                </AccordionPanel>
              </Box>
            )}
          </AccordionItem>
        </Accordion>
        {isInitialPage && (
          <Button
            onClick={() => handleInitialize(controllerMap)}
            isLoading={isInitializing}
            mt={4}
            colorScheme="blue"
          >
            Initialize
          </Button>
        )}
      </Stack>
    </Stack>
  )
}

export default Controllers
