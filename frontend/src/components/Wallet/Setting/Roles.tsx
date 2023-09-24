import { CloseIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  CardBody,
  Checkbox,
  CloseButton,
  FormControl,
  FormLabel,
  Grid,
  HStack,
  IconButton,
  Input,
  Select,
  Stack,
  Table,
  TableContainer,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr,
  VStack
} from "@chakra-ui/react"
import Loading from "components/Loading"
import {
  OperationAccess,
  OperationEnum,
  Role
} from "declarations/b3_wallet/b3_wallet.did"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"

const operations = [
  "UnhideAccount",
  "EvmDeployContract",
  "IcpTransfer",
  "EvmSignRawTransaction",
  "EvmSignMessage",
  "UpdateCanisterSettings",
  "RenameAccount",
  "AddUser",
  "EvmSignTranscation",
  "EvmTransferErc20",
  "SendToken",
  "HideAccount",
  "UpgradeCanister",
  "TopUpTransfer",
  "BtcTransfer",
  "RemoveUser",
  "RemoveAccount",
  "CreateAccount",
  "EvmTransfer"
] as const

const accessLevels = ["ReadOnly", "Limited", "Canister", "FullAccess"] as const

type AccessLevel = (typeof accessLevels)[number]

type Operations = (typeof operations)[number]

type OperationValidUntil = {
  [key in Operations]?: bigint
}

interface RolesProps {
  actor: B3Wallet
}

const Roles: React.FC<RolesProps> = ({ actor, ...rest }) => {
  const [loading, setLoading] = useState(false)

  const [roles, setRoles] = useState<Array<[bigint, Role]>>([])
  const [roleName, setRoleName] = useState<string>("")

  const [accessLevel, setAccessLevel] = useState<AccessLevel>()
  const [selectedOperations, setSelectedOperations] =
    useState<OperationValidUntil>({})

  const fetchRoles = async () => {
    if (!actor) return

    setLoading(true)
    actor
      .get_roles()
      .then(roles => {
        setLoading(false)
        setRoles(roles)
      })
      .catch(e => {
        setLoading(false)
        console.log(e)
      })
  }

  useEffect(() => {
    fetchRoles()
  }, [actor])

  const handleOperationChange = (operation: Operations, checked: boolean) => {
    setSelectedOperations(prev => {
      if (checked) {
        return {
          ...prev,
          [operation]: 0
        }
      }

      const newState = Object.keys(prev).reduce((acc, key) => {
        if (key !== operation) {
          acc[key as Operations] = prev[key as Operations]
        }
        return acc
      }, {} as OperationValidUntil)

      return newState
    })
  }

  const handleDateChange = (operation: Operations, date: number) => {
    setSelectedOperations(prev => ({
      ...prev,
      [operation]: BigInt(date) * BigInt(1000)
    }))
  }

  const handleSubmit = () => {
    if (!accessLevel || !actor || !roleName) {
      return
    }

    let access_level: Role["access_level"]

    switch (accessLevel) {
      case "ReadOnly":
        access_level = {
          ReadOnly: null
        }
        break
      case "Limited":
        access_level = {
          Limited: Object.entries(selectedOperations).reduce(
            (acc, [operation, validUntil]) => {
              acc.push({
                operation: { [operation]: null } as OperationEnum,
                valid_until: validUntil ? [validUntil] : []
              })

              return acc
            },
            [] as Array<OperationAccess>
          )
        }
        break
      case "Canister":
        access_level = {
          Canister: null
        }
        break
      case "FullAccess":
        access_level = {
          FullAccess: null
        }
        break
    }

    const role: Role = {
      access_level,
      name: roleName
    }

    console.log(role)

    setLoading(true)
    actor
      .role_add(role)
      .then(() => {
        fetchRoles()
        console.log("Role created")
      })
      .catch(e => {
        console.error(e)
      })
  }

  const removeRole = (id: bigint) => {
    if (!actor) {
      return
    }

    setLoading(true)

    actor
      .role_remove(id)
      .then(() => {
        fetchRoles()
        console.log("Role removed")
      })
      .catch(e => {
        setLoading(false)
        console.error(e)
      })
  }

  return (
    <Stack
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
      {...rest}
    >
      {loading && <Loading title="Loading Roles" />}
      <Accordion allowToggle>
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
                  Roles
                </Text>
                <Stack fontSize="sm" fontWeight="semibold">
                  <Stack direction="row" align="center">
                    {isExpanded && (
                      <IconButton
                        aria-label="Refresh"
                        icon={<RepeatIcon />}
                        onClick={fetchRoles}
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
                          <Th>Name</Th>
                          <Th>Role</Th>
                          <Th>Access Level</Th>
                          <Th>X</Th>
                        </Tr>
                      </Thead>
                      <Tbody w="100%">
                        {roles.map(([id, { access_level, name }], index) => {
                          const accessLevel = Object.keys(access_level)[0]
                          return (
                            <Tr key={index}>
                              <Td>{name}</Td>
                              <Td>{accessLevel}</Td>
                              <Td>
                                {accessLevel === "Limited"
                                  ? access_level[accessLevel].map(
                                      ({ operation, valid_until }, i) => (
                                        <Box key={i}>
                                          {Object.keys(operation)[0]}{" "}
                                          {valid_until.length > 0 && (
                                            <Box
                                              as="span"
                                              color="gray.500"
                                              fontSize="xs"
                                            >
                                              {new Date(
                                                Number(
                                                  valid_until[0] / BigInt(1000)
                                                )
                                              ).toLocaleDateString()}
                                            </Box>
                                          )}
                                        </Box>
                                      )
                                    )
                                  : accessLevel}
                              </Td>
                              <Td textAlign="right">
                                <CloseButton
                                  textAlign="right"
                                  color="red"
                                  onClick={() => removeRole(id)}
                                />
                              </Td>
                            </Tr>
                          )
                        })}
                      </Tbody>
                    </Table>
                  </TableContainer>
                  <HStack spacing={2} w="100%" pt={2}>
                    <Input
                      placeholder="Role name"
                      onChange={e => setRoleName(e.target.value)}
                    />
                    <Select
                      placeholder="Select option"
                      onChange={e =>
                        setAccessLevel(e.target.value as AccessLevel)
                      }
                    >
                      {accessLevels.map(level => (
                        <option key={level} value={level}>
                          {level}
                        </option>
                      ))}
                    </Select>
                  </HStack>
                  {accessLevel === "Limited" && (
                    <VStack spacing={2} width="100%">
                      <FormControl id="operations">
                        <FormLabel>Select Operations</FormLabel>
                        <Grid templateColumns="repeat(2, 1fr)">
                          {operations.map(op => (
                            <Checkbox
                              key={op}
                              value={op}
                              isChecked={selectedOperations[op] !== undefined}
                              onChange={e =>
                                handleOperationChange(op, e.target.checked)
                              }
                            >
                              {op}
                            </Checkbox>
                          ))}
                        </Grid>
                      </FormControl>
                      <Box
                        width="100%"
                        overflowY="auto"
                        border="1px solid"
                        borderRadius={4}
                        borderColor="gray.200"
                      >
                        <HStack
                          spacing={0}
                          width="100%"
                          fontWeight="bold"
                          borderBottom={2}
                          borderColor="gray.200"
                          borderStyle="solid"
                          padding={2}
                        >
                          <Box flex={4}>Operation</Box>
                          <Box flex={7}>Valid Until (Optional)</Box>
                          <Box flex={1} />
                        </HStack>
                        {Object.keys(selectedOperations).map(operation => (
                          <HStack
                            key={operation}
                            overflow="hidden"
                            justifyContent="center"
                            alignItems="center"
                            padding={2}
                            spacing={2}
                          >
                            <Box
                              fontWeight="bold"
                              overflow="hidden"
                              whiteSpace="nowrap"
                              textOverflow="ellipsis"
                              flex={4}
                            >
                              {operation}
                            </Box>
                            <Box flex={7}>
                              <FormControl id={`validUntil-${operation}`}>
                                <Input
                                  type="date"
                                  onChange={e =>
                                    handleDateChange(
                                      operation as Operations,
                                      e.target.valueAsNumber
                                    )
                                  }
                                />
                              </FormControl>
                            </Box>
                            <Box
                              textAlign="right"
                              width="10px"
                              p={0}
                              pr={2}
                              flex={1}
                            >
                              <IconButton
                                padding={0}
                                margin={0}
                                aria-label="Delete"
                                size="xs"
                                icon={<CloseIcon />}
                                colorScheme="red"
                                onClick={() =>
                                  handleOperationChange(
                                    operation as Operations,
                                    false
                                  )
                                }
                              />
                            </Box>
                          </HStack>
                        ))}
                      </Box>
                    </VStack>
                  )}
                  <Button
                    colorScheme="blue"
                    onClick={handleSubmit}
                    w="100%"
                    mt={2}
                  >
                    Create Role
                  </Button>
                </CardBody>
              </AccordionPanel>
            </Box>
          )}
        </AccordionItem>
      </Accordion>
    </Stack>
  )
}

export default Roles
