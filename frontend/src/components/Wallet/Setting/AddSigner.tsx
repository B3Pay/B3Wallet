import {
  Box,
  Button,
  FormControl,
  Input,
  Select,
  Stack
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { Roles } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3Wallet } from "service/actor"

interface AddSignerProps {
  actor: B3Wallet
}

enum RoleEnum {
  User = "User",
  Canister = "Canister",
  Admin = "Admin"
}

type Role = keyof typeof RoleEnum

const AddSigner: React.FC<AddSignerProps> = ({ actor }) => {
  const [principal, setPrincipal] = useState("")
  const [role, setRole] = useState<Role>()
  const errorToast = useToastMessage()

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
    // Here you would send the data to your backend
    console.log({ principal, role })

    const roles = {
      [role]: null
    } as Roles

    // Add the signer
    actor
      .signer_add(Principal.fromText(principal), roles)
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
  )
}

export default AddSigner
