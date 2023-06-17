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
  Select,
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
import { Roles, Signer } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3Wallet } from "service/actor"
import Address from "../Address"

export type SignerMap = Array<[Principal, Signer]>

interface SignerProps extends StackProps {
  actor: B3Wallet
  signers: SignerMap
  isInitialPage?: boolean
  refetch: () => void
}

enum RoleEnum {
  User = "User",
  Canister = "Canister",
  Admin = "Admin"
}

type Role = keyof typeof RoleEnum

const Signers: React.FC<SignerProps> = ({
  actor,
  signers,
  refetch,
  isInitialPage,
  ...rest
}) => {
  const [loading, setLoading] = useState(false)
  const [principal, setPrincipal] = useState("")
  const [role, setRole] = useState<Role | "select">()
  const errorToast = useToastMessage()

  // Remove a user
  const removeSigner = async (signerId: Principal) => {
    setLoading(true)

    await actor
      .signer_remove(signerId)
      .then(() => {
        refetch()
        errorToast({
          title: "Signer Removed.",
          description: `Signer ${signerId.toString()} has been removed.`,
          status: "success",
          duration: 9000,
          isClosable: true
        })
      })
      .catch(e => {
        console.log(e)
        errorToast({
          title: "Error removing signer.",
          description: `Please try again`,
          status: "error",
          duration: 9000,
          isClosable: true
        })
      })

    setLoading(false)
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!role) {
      errorToast({
        title: "Role not selected.",
        description: `Please select a role`,
        status: "error",
        duration: 9000,
        isClosable: true
      })
      return
    }

    const roles = {
      [role]: null
    } as Roles

    let signerId: Principal
    setLoading(true)

    try {
      signerId = Principal.fromText(principal)
    } catch (e) {
      console.log(e)

      return errorToast({
        title: "Invalid principal.",
        description: `Please enter a valid principal`,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    }

    // Add the signer
    await actor
      .signer_add(signerId, roles)
      .then(() => {
        errorToast({
          title: "Signer added.",
          description: `Principal ${principal} added with role ${role}`,
          status: "success",
          duration: 9000,
          isClosable: true
        })

        // Clear the form
        setPrincipal("")
        setRole("select")
        refetch()
      })
      .catch(e => {
        console.log(e)
        errorToast({
          title: "Error adding signer.",
          description: `Please try again`,
          status: "error",
          duration: 9000,
          isClosable: true
        })
      })

    setLoading(false)
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
      {...rest}
    >
      {(!signers || loading) && <Loading title="Loading signers" />}
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
                  Signers
                </Text>
                <Stack fontSize="sm" fontWeight="semibold">
                  <Stack direction="row" align="center">
                    {isExpanded && (
                      <IconButton
                        aria-label="Refresh"
                        icon={<RepeatIcon />}
                        onClick={refetch}
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
                          <Th>Signer ID</Th>
                          <Th>Role</Th>
                          <Th></Th>
                        </Tr>
                      </Thead>
                      <Tbody>
                        {signers?.map(([userId, { role }], index) => (
                          <Tr key={index}>
                            <Td>
                              <Address address={userId.toString()} noIcon />
                            </Td>
                            <Td>{Object.keys(role)[0]}</Td>
                            <Td>
                              <CloseButton
                                color="red"
                                onClick={() => removeSigner(userId)}
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
                      <FormControl isRequired flex={4}>
                        <Select
                          value={role}
                          onChange={e => {
                            const role = e.target.value as Role

                            setRole(role)
                          }}
                        >
                          <option value={"select"}>Select Role</option>
                          {Object.keys(RoleEnum).map((role, i) => (
                            <option key={i} value={role}>
                              {role}
                            </option>
                          ))}
                        </Select>
                      </FormControl>
                      <Button colorScheme="orange" type="submit" flex={3}>
                        Add Signer
                      </Button>
                    </Stack>
                  </Box>
                </CardBody>
              </AccordionPanel>
            </Box>
          )}
        </AccordionItem>
      </Accordion>
    </Stack>
  )
}

export default Signers
