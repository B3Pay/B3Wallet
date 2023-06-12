import { Button, Input, Select, Stack } from "@chakra-ui/react"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3Wallet } from "service/actor"

interface RestoreAccountProps {
  actor: B3Wallet
  fetchAccounts: () => void
}

const RestoreAccount: React.FC<RestoreAccountProps> = ({
  actor,
  fetchAccounts
}) => {
  const [loading, setLoading] = useState(false)
  const [nonce, setNonce] = useState<bigint>(0n)
  const { errorToast } = useToastMessage()

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

  const createAccount = async () => {
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
        errorToast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoading(false)
      })
  }

  return (
    <Stack direction="row" justify="space-between" align="center">
      <Select
        flex={6}
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
        flex={2}
        type="number"
        placeholder="Nonce"
        value={nonce.toString()}
        onChange={onChangeName}
      />
      <Button
        onClick={createAccount}
        isLoading={loading}
        flex={4}
        colorScheme="green"
      >
        Restore
      </Button>
    </Stack>
  )
}

export default RestoreAccount
