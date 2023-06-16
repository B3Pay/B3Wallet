import {
  Button,
  CardBody,
  CardHeader,
  CloseButton,
  FormControl,
  FormLabel,
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
import { WalletSettingsAndSigners } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3Wallet } from "service/actor"
import Address from "./Address"

interface InitialSetupProps extends WalletSettingsAndSigners {
  actor: B3Wallet
}

const InitialSetup: React.FC<InitialSetupProps> = ({
  actor,
  settings,
  signers
}) => {
  const [isInitializing, setIsInitializing] = useState(false)
  const [newName, setNewName] = useState("")
  const [newPublicKey, setNewPublicKey] = useState("")

  // Add a user
  const addUser = () => {
    setNewName("")
    setNewPublicKey("")
  }

  // Remove a user
  const removeUser = index => {}

  const handleInitialize = async () => {
    setIsInitializing(true)
    try {
      await actor.init_wallet()
    } catch (err) {
      console.error(err)
    } finally {
      setIsInitializing(false)
    }
  }

  return (
    <Stack>
      <Text fontSize="xl" fontWeight="bold" my={2} textAlign="center">
        Initial Setup
      </Text>
      <Stack
        direction="column"
        borderWidth="1px"
        borderRadius="lg"
        overflow="hidden"
      >
        <CardHeader pb={2}>
          <Text fontSize="md" fontWeight="bold">
            Signers
          </Text>
        </CardHeader>
        <CardBody borderTop="1px" borderColor="gray.200">
          <Stack spacing={2}>
            <Text mb={4}>
              Add or remove signer for your wallet. You can add more signer
              later.
            </Text>
            <Text mb={4} fontSize="sm">
              Note: The system canister is a connected canister by default, it
              only have access to some data information, you can remove it if
              you want.
            </Text>
            <TableContainer>
              <Table variant="simple">
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
                        <CloseButton onClick={() => removeUser(index)} />
                      </Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            </TableContainer>
            <FormControl>
              <FormLabel>Name</FormLabel>
              <Input
                value={newName}
                onChange={e => setNewName(e.target.value)}
              />

              <FormLabel>Public Key</FormLabel>
              <Input
                value={newPublicKey}
                onChange={e => setNewPublicKey(e.target.value)}
              />

              <Button onClick={addUser}>Add User</Button>
            </FormControl>
          </Stack>
        </CardBody>
      </Stack>
      <Stack
        direction="column"
        borderWidth="1px"
        borderRadius="lg"
        overflow="hidden"
      >
        <CardHeader pb={2}>
          <Text fontSize="md" fontWeight="bold">
            Controllers
          </Text>
        </CardHeader>
        <CardBody borderTop="1px" borderColor="gray.200">
          <Stack spacing={2}>
            <Text mb={4}>
              Add or remove controller for your wallet. You can add more
              controller later.
            </Text>
            <Text mb={4} fontSize="sm">
              Note: the wallet canister itself always is a controller.
            </Text>
            <TableContainer>
              <Table variant="simple">
                <Thead>
                  <Tr>
                    <Th>Controller</Th>
                    <Th></Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {settings?.controllers?.map((controller, index) => (
                    <Tr key={index}>
                      <Td>
                        <Address address={controller.toString()} noIcon />
                      </Td>
                      <Td>
                        <CloseButton onClick={() => removeUser(index)} />
                      </Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            </TableContainer>
            <FormControl>
              <FormLabel>Name</FormLabel>
              <Input
                value={newName}
                onChange={e => setNewName(e.target.value)}
              />
              <FormLabel>Public Key</FormLabel>
              <Input
                value={newPublicKey}
                onChange={e => setNewPublicKey(e.target.value)}
              />
              <Button onClick={addUser}>Add User</Button>
            </FormControl>
          </Stack>
        </CardBody>
      </Stack>
      <Button
        onClick={handleInitialize}
        isLoading={isInitializing}
        mt={4}
        colorScheme="blue"
      >
        Initialize
      </Button>
    </Stack>
  )
}

export default InitialSetup
