/* eslint-disable @next/next/no-img-element */
import Head from "next/head"

import CreateAccount from "components/CreateAccount"
import EthAccount from "components/EthAccount"
import { Account } from "declarations/b3_user/b3_user.did"
import useAuthClient from "hooks/useAuthClient"
import { useCallback, useEffect, useState } from "react"
import { B3User, makeB3UserActor } from "service/actor"
import styles from "styles/Home.module.css"

function HomePage() {
  const { isAuthenticated, authClient, login, logout, systemActor } =
    useAuthClient()

  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string>()

  const [accounts, setAccounts] = useState<Account[]>([])
  const [actor, setActor] = useState<B3User>()

  const [version, setVersion] = useState<string>("0.0.0")

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      if (!canisterId || !authClient) {
        console.log("no canisterId or authClient")
        return
      }
      setLoading(true)

      const userActor = makeB3UserActor(canisterId, authClient.getIdentity())

      const version = await userActor.version()

      setVersion(version)
      setActor(userActor)
      setLoading(false)
    },
    [authClient]
  )

  const fetchCanisterId = useCallback(async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)

    const control = await systemActor.get_user_control()

    if (control.length === 0) {
      setError("No user control found")
      setLoading(false)
      return
    }

    const canisterId = control[0].user_control_id.toString()

    fetchUserActor(canisterId)
    setLoading(false)
  }, [authClient, systemActor, fetchUserActor])

  const fetchAccounts = useCallback(async () => {
    if (!actor) {
      console.log("no actor")
      return
    }
    setLoading(true)

    const accounts = await actor.get_accounts()

    setAccounts(accounts)
    setLoading(false)
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

  useEffect(() => {
    fetchCanisterId()
  }, [fetchCanisterId])

  const createUserHandler = async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)
    const userControl = await systemActor.create_user_control()

    if ("Err" in userControl) {
      setError(userControl.Err)
      setLoading(false)
      return
    }

    fetchUserActor(userControl.Ok.user_control_id.toString())
    setLoading(false)
  }

  return (
    <div className={styles.container}>
      <Head>
        <title>B3Pay System</title>
      </Head>
      <main className={styles.main}>
        <h3 className={styles.title}>Welcome to B3Pay User Wallet!</h3>
        <img src="/logo.png" alt="DFINITY logo" className={styles.logo} />
        {loading && <p>Loading...</p>}
        {error && <p>{error}</p>}
        {!isAuthenticated ? (
          <section
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center"
            }}
          >
            <button onClick={login}>Login</button>
          </section>
        ) : actor ? (
          <section>
            <CreateAccount actor={actor} fetchAccounts={fetchCanisterId} />
            <label>Accounts: &nbsp;</label>
            {accounts.map((account, index) => (
              <EthAccount key={index} {...account} actor={actor} />
            ))}
            <button onClick={logout}>Logout</button>
          </section>
        ) : (
          <section
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center"
            }}
          >
            <button onClick={() => createUserHandler()}>Create User</button>
            <button onClick={logout}>Logout</button>
          </section>
        )}
      </main>
      {/* add version of canister wasm */}
      <footer className={styles.footer}>
        <p>Version: {version}</p>
      </footer>
    </div>
  )
}

export default HomePage
