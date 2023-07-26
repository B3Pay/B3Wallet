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
  Grid,
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
import {
  AddSigner,
  Signer,
  SignerRoles
} from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useMemo, useState } from "react"
import { B3Wallet } from "service"
import Address from "../Address"

interface SignerWithRole extends Omit<Signer, "role"> {
  role: Role
  id: Principal
}

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
  const [role, setRole] = useState<Role | "select">("select")
  const [name, setName] = useState("")

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
    } as SignerRoles

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

    const args: AddSigner = {
      signer_id: signerId,
      role: roles,
      expires_at: [],
      name,
      threshold: []
    }

    // Add the signer
    await actor
      .request_add_signer(args, "Demo wallet", [])
      .then(() => {
        errorToast({
          title: "Request sent.",
          description: `Request to add signer ${signerId.toString()} has been sent.`,
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

  const signerSorted = useMemo(() => {
    if (!signers) return []

    return signers
      .map(([id, signer]) => ({
        id,
        ...signer,
        role: Object.keys(signer.role)[0] as Role
      }))
      .sort((a, b) => a.role.localeCompare(b.role))
  }, [signers])

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
                          <Th>Name</Th>
                          <Th></Th>
                        </Tr>
                      </Thead>
                      <Tbody>
                        {signerSorted.map(({ id, role, name }, index) => (
                          <Tr key={index}>
                            <Td>
                              <Address address={id.toString()} noIcon />
                            </Td>
                            <Td>{role}</Td>
                            <Td>{name}</Td>
                            <Td>
                              <CloseButton
                                color="red"
                                onClick={() => removeSigner(id)}
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
                    pb={0}
                    borderTop="1px"
                    borderColor="gray.200"
                  >
                    <Grid
                      templateColumns={{
                        base: "repeat(2, 1fr)",
                        md: "repeat(4, 1fr)"
                      }}
                      gap={2}
                    >
                      <FormControl isRequired>
                        <Input
                          value={principal}
                          onChange={e => setPrincipal(e.target.value)}
                          placeholder="Principal"
                        />
                      </FormControl>
                      <FormControl isRequired>
                        <Input
                          value={name}
                          onChange={e => setName(e.target.value)}
                          placeholder="Name"
                        />
                      </FormControl>
                      <FormControl isRequired>
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
                      <Button colorScheme="orange" type="submit">
                        Add {role !== "select" ? role : "Signer"}
                      </Button>
                    </Grid>
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
