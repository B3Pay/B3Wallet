import { Account, Result } from "declarations/b3_user/b3_user.did"
import { useEffect, useState } from "react"

// Dfinity
import { B3User, makeB3UserActor } from "service/actor"

const CreateAccount = () => {
  const [name, setName] = useState<string>()
  const [loading, setLoading] = useState("")
  const [response, setResponse] = useState<Result>()
  const [actor, setActor] = useState<B3User>()
  const [accounts, setAccounts] = useState<Account[]>([])

  useEffect(() => {
    const b3UserActor = makeB3UserActor()
    setActor(b3UserActor)
  }, [])

  useEffect(() => {
    actor?.get_accounts().then((accounts: any) => {
      setAccounts(accounts)
    })
  }, [actor])

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    const newName = e.target.value
    setName(newName)
  }

  async function createAccount() {
    setResponse(undefined)
    setLoading("Loading...")

    const b3UserActor = makeB3UserActor()
    const greeting = await b3UserActor.create_account(
      { Development: null },
      name ? [name] : []
    )

    setLoading("")
    setResponse(greeting)
  }

  console.log("acciybt response", response)

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
        <button onClick={createAccount}>Create</button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading}
        {JSON.stringify(response)}
      </section>
      <section>
        <label>Accounts: &nbsp;</label>
        {accounts.map(account => {
          return (
            <div key={account.name}>
              {account.name}: {account.public_key.address}
            </div>
          )
        })}
      </section>
    </div>
  )
}

export default CreateAccount
