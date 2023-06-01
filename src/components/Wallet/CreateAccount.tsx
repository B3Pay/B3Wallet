import {
  Box,
  Button,
  FormControl,
  Input,
  Select,
  Stack
} from "@chakra-ui/react"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import { useState } from "react"
import { B3Wallet } from "service/actor"

interface CreateAccountProps {
  actor: B3Wallet
  fetchAccounts: () => void
}

const CreateAccount: React.FC<CreateAccountProps> = ({
  actor,
  fetchAccounts
}) => {
  const [name, setName] = useState<string>()
  const [environment, setEnvironment] = useState<Environment>(
    IS_LOCAL
      ? {
          Development: null
        }
      : {
          Production: null
        }
  )

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    const newName = e.target.value
    setName(newName)
  }

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()

    await actor.request_create_account(
      {
        name: name ? [name] : [],
        env: [environment]
      },
      []
    )

    fetchAccounts()
  }

  return (
    <Box as="form" onSubmit={handleSubmit}>
      <Stack alignItems="center" justify="space-between" direction="row">
        <FormControl flex={5}>
          <Input
            id="name"
            alt="Name"
            type="text"
            placeholder="Name"
            value={name}
            onChange={onChangeName}
          />
        </FormControl>
        <FormControl isRequired flex={5}>
          <Select
            value={Object.keys(environment)[0]}
            onChange={e => {
              const env = e.target.value
              setEnvironment({ [env]: null } as Environment)
            }}
          >
            <option value="Development">Development</option>
            <option value="Production">Production</option>
            <option value="Staging">Staging</option>
          </Select>
        </FormControl>
        <Button colorScheme="orange" type="submit" flex={2}>
          Create
        </Button>
      </Stack>
    </Box>
  )
}

export default CreateAccount
