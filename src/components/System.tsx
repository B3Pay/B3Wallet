import { Principal } from "@dfinity/principal"
import React, { useEffect, useState } from "react"
import { callActor, useActorMethod } from "service/system"

interface SystemProps {}

const System: React.FC<SystemProps> = ({}) => {
  const { call, data, error, loading } = useActorMethod(
    "install_wallet_canister"
  )

  useEffect(() => {
    callActor("get_user_states").then(res => {
      console.log(res)
    })
  }, [])

  const [canisterId, setCanisterId] = useState("")

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    const newName = e.target.value
    setCanisterId(newName)
  }

  const walletHandler = async () => {
    const walletCanisterId = Principal.fromText(canisterId)

    const res = await call(walletCanisterId)
    console.log(res)
  }
  return (
    <div>
      <section>
        <h2>B3Wallet</h2>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input
          id="name"
          alt="Name"
          type="text"
          value={canisterId}
          onChange={onChangeName}
        />
        <button onClick={walletHandler}>Create Wallet</button>
      </section>
      <section>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {JSON.stringify(error)}</span> : null}
        {data && <span>{JSON.stringify(data)}</span>}
      </section>
    </div>
  )
}

export default System
