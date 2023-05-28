import { Button, Flex, Input, Select, Stack } from "@chakra-ui/react"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import { useState } from "react"
import { B3User } from "service/actor"

interface CreateAccountProps {
  actor: B3User
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

  async function createAccount() {
    if (!actor) {
      return
    }

    await actor.account_create([environment], name ? [name] : [])

    fetchAccounts()
  }

  return (
    <Stack
      spacing="2"
      p={1}
      direction="row"
      justify="space-between"
      align="center"
    >
      <Input
        id="name"
        alt="Name"
        type="text"
        placeholder="Name"
        value={name}
        onChange={onChangeName}
      />
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
      <Flex flex="1">
        <Button onClick={createAccount}>Create</Button>
      </Flex>
    </Stack>
  )
}

export default CreateAccount
