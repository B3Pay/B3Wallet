import { Account, Environment, Result } from "declarations/b3_user/b3_user.did"
import useAuthClient from "hooks/useAuthClient"
import { useCallback, useEffect, useState } from "react"
import EthAccount from "./EthAccount"

const CreateAccount = () => {
  const [name, setName] = useState<string>()
  const [environment, setEnvironment] = useState<Environment>({
    Production: null
  })

  const [loading, setLoading] = useState("")
  const [response, setResponse] = useState<Result>()
  const [accounts, setAccounts] = useState<Account[]>([])
  const { isAuthenticated, login, logout, actor } = useAuthClient()

  const fetchAccounts = useCallback(async () => {
    if (!actor) {
      return
    }

    const accounts = await actor.get_accounts()

    setAccounts(accounts)
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

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
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading}
        {JSON.stringify(response)}
      </section>
      {!isAuthenticated ? (
        <section>
          <button onClick={login}>Login</button>
        </section>
      ) : (
        <section>
          <label>Accounts: &nbsp;</label>
          {accounts.map((account, index) => (
            <EthAccount key={index} {...account} actor={actor} />
          ))}
          <button onClick={logout}>Logout</button>
        </section>
      )}
    </div>
  )
}

export default CreateAccount
