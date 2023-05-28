import { Button, Flex, Input, Select, Stack } from "@chakra-ui/react"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import { useState } from "react"
import { B3User } from "service/actor"

interface RestoreAccountProps {
  actor: B3User
  fetchAccounts: () => void
}

const RestoreAccount: React.FC<RestoreAccountProps> = ({
  actor,
  fetchAccounts
}) => {
  const [loading, setLoading] = useState(false)
  const [nonce, setNonce] = useState<bigint>(0n)
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
    const newName = BigInt(e.target.value)
    setNonce(newName)
  }

  async function createAccount() {
    if (!actor) {
      return
    }
    setLoading(true)
    actor
      .account_restore(environment, nonce)
      .then(() => {
        setLoading(false)
        fetchAccounts()
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
  }

  return (
    <Stack spacing="2" direction="row" justify="space-between" align="center">
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
      <Input
        id="nonce"
        alt="Name"
        type="number"
        placeholder="Nonce"
        value={nonce.toString()}
        onChange={onChangeName}
      />
      <Flex flex="1">
        <Button onClick={createAccount} isLoading={loading}>
          Restore
        </Button>
      </Flex>
    </Stack>
  )
}

export default RestoreAccount
