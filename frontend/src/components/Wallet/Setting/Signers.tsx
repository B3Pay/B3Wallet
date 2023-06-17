import { RepeatIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
  CardBody,
  CardHeader,
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
  ...rest
}) => {
  const [loading, setLoading] = useState(false)
  const [principal, setPrincipal] = useState("")
  const [role, setRole] = useState<Role>()
  const errorToast = useToastMessage()

  // Remove a user
  const removeSigner = (signerId: Principal) => {
    actor
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
  }

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
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
    actor
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
        setRole(undefined)
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
      {!signers && <Loading title="Loading signers" />}
      <CardHeader pb={2}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontSize="md" fontWeight="bold">
            Signers
          </Text>
          <Stack fontSize="sm" fontWeight="semibold">
            {loading ? (
              <Text>Loading...</Text>
            ) : (
              <Stack direction="row" align="center">
                <IconButton
                  aria-label="Refresh"
                  icon={<RepeatIcon />}
                  onClick={refetch}
                  size="xs"
                />
              </Stack>
            )}
          </Stack>
        </Stack>
      </CardHeader>
      <CardBody borderTop="1px" borderColor="gray.200" m={0} p={0}>
        <TableContainer minH={75}>
          <Table size="sm">
            <Thead>
              <Tr>
                <Th>Signer</Th>
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
          <Stack alignItems="center" justify="space-between" direction="row">
            <FormControl isRequired flex={5}>
              <Input
                value={principal}
                onChange={e => setPrincipal(e.target.value)}
                placeholder="Principal"
              />
            </FormControl>
            <FormControl isRequired flex={4}>
              <Select
                placeholder="Select role"
                value={role}
                onChange={e => {
                  const role = e.target.value as Role

                  setRole(role)
                }}
              >
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
    </Stack>
  )
}

export default Signers
