import {
  Box,
  Button,
  FormControl,
  Input,
  Select,
  Stack,
  useToast
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { Roles } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3Wallet } from "service/actor"

interface AddSignerProps {
  actor: B3Wallet
}

type Role = "User" | "Canister" | "Admin"

const AddSigner: React.FC<AddSignerProps> = ({ actor }) => {
  const [principal, setPrincipal] = useState("")
  const [role, setRole] = useState<Role>()
  const toast = useToast()

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!role) {
      toast({
        title: "Role not selected.",
        description: `Please select a role`,
        status: "error",
        duration: 9000,
        isClosable: true
      })
      return
    }
    // Here you would send the data to your backend
    console.log({ principal, role })

    const roles = {
      [role]: null
    } as Roles

    // Add the signer
    actor
      .signer_add(Principal.fromText(principal), roles)
      .then(() => {
        toast({
          title: "Signer added.",
          description: `Principal ${principal} added with role ${role}`,
          status: "success",
          duration: 9000,
          isClosable: true
        })

        // Clear the form
        setPrincipal("")
        setRole(undefined)
      })
      .catch(e => {
        console.log(e)
        toast({
          title: "Error adding signer.",
          description: `Please try again`,
          status: "error",
          duration: 9000,
          isClosable: true
        })
      })
  }

  return (
    <Box as="form" onSubmit={handleSubmit}>
      <Stack alignItems="center" justify="space-between" direction="row">
        <FormControl isRequired flex={4}>
          <Input
            value={principal}
            onChange={e => setPrincipal(e.target.value)}
            placeholder="Principal"
          />
        </FormControl>
        <FormControl isRequired flex={5}>
          <Select
            placeholder="Select role"
            value={role}
            onChange={e => {
              const role = e.target.value as Role

              setRole(role)
            }}
          >
            <option value="User">User</option>
            <option value="Canister">Canister</option>
            <option value="Admin">Admin</option>
          </Select>
        </FormControl>
        <Button colorScheme="orange" type="submit" flex={3}>
          Add Signer
        </Button>
      </Stack>
    </Box>
  )
}

export default AddSigner
