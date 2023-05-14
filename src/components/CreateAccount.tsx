import {
  Environment,
  SignerAccount
} from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import { useState } from "react"
import { B3User } from "service/actor"
import { Response } from "./Response"

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

  const [loading, setLoading] = useState("")
  const [response, setResponse] = useState<SignerAccount>()

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    const newName = e.target.value
    setName(newName)
  }

  async function createAccount() {
    if (!actor) {
      return
    }

    setResponse(undefined)
    setLoading("Loading...")

    const account = await actor.create_account(
      [environment],
      name ? [name] : []
    )

    fetchAccounts()

    setLoading("")
    setResponse(account)
  }

  return (
    <div>
      <section>
        <h2>MultiChain Wallet</h2>
        <label htmlFor="name">Enter Account name: &nbsp;</label>
        <input
          id="name"
          alt="Name"
          type="text"
          value={name}
          onChange={onChangeName}
        />
        <select
          value={Object.keys(environment)[0]}
          onChange={e => {
            const env = e.target.value

            setEnvironment({ [env]: null } as Environment)
          }}
        >
          <option value="Development">Development</option>
          <option value="Production">Production</option>
          <option value="Staging">Staging</option>
        </select>
        <button onClick={createAccount}>Create</button>
        <Response response={response} loading={loading} />
      </section>
    </div>
  )
}

export default CreateAccount
