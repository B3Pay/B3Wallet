/* eslint-disable @next/next/no-img-element */
import { Principal } from "@dfinity/principal"
import Account from "components/Account"
import CreateAccount from "components/CreateAccount"
import { Footer } from "components/Footer"
import { Response } from "components/Response"
import {
  WalletAccount,
  WalletCanisterStatus
} from "declarations/b3_wallet/b3_wallet.did"
import useAuthClient from "hooks/useAuthClient"
import Head from "next/head"
import { useCallback, useEffect, useState } from "react"
import { B3User, makeB3UserActor } from "service/actor"
import styles from "styles/Home.module.css"

function HomePage() {
  const { isAuthenticated, authClient, login, logout, systemActor } =
    useAuthClient()

  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string>()
  const [version, setVersion] = useState<string>("")

  const [status, setStatus] = useState<WalletCanisterStatus>()
  const [accounts, setAccounts] = useState<WalletAccount[]>([])
  const [actor, setActor] = useState<B3User>()
  const [canisterId, setCanisterId] = useState<string>("")

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      if (!canisterId || !authClient) {
        console.log("no canisterId or authClient")
        return
      }
      const userActor = makeB3UserActor(canisterId, authClient.getIdentity())

      userActor
        .version()
        .then(async version => {
          setLoading(true)

          const status = await userActor.status()

          setStatus(status)

          setVersion(version)
          setActor(userActor)
          setLoading(false)
        })
        .catch(e => {
          console.log(e)
          setLoading(false)
        })
    },
    [authClient]
  )

  const fetchCanisterId = useCallback(async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)

    systemActor
      .get_canister()
      .then(({ canister_id }) => {
        const canisterId = canister_id.toString()

        setCanisterId(canisterId)
        fetchUserActor(canisterId)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
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

  const createUser = async () => {
    setError(undefined)
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)
    const userControl = await systemActor.create_wallet_canister()

    if ("Err" in userControl) {
      setLoading(false)
      return console.log(userControl.Err)
    }

    fetchUserActor(userControl.Ok.canister_id.toString())
    setLoading(false)
  }

  const installCanister = async () => {
    setError(undefined)
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)

    const canisterPrincipal = Principal.fromText(canisterId)

    const userControl = await systemActor.install_wallet_canister([
      canisterPrincipal
    ])

    if ("Err" in userControl) {
      setLoading(false)
      return setError(userControl.Err)
    }

    fetchUserActor(userControl.Ok.canister_id.toString())
    setLoading(false)
  }

  const reset_account = async () => {
    setError(undefined)
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(true)

    const result = await actor.reset_wallet()

    console.log(result)

    fetchAccounts()

    setLoading(false)
  }

  return (
    <div className={styles.container}>
      <Head>
        <title>B3Wallet</title>
      </Head>
      {loading && (
        <div
          style={{
            position: "fixed",
            top: 0,
            left: 0,
            width: "100%",
            height: "100%",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            backgroundColor: "rgba(0,0,0,0.5)",
            zIndex: 1000
          }}
        >
          Loading...
        </div>
      )}

      <main className={styles.main}>
        {error && (
          <p
            style={{
              color: "red"
            }}
          >
            {error}
          </p>
        )}
        <div>
          <label>Canister Id: &nbsp;</label>
          <span>{canisterId.toString()}</span>
        </div>
        {!isAuthenticated ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center"
            }}
          >
            <button onClick={login}>Login</button>
          </div>
        ) : actor ? (
          <div>
            <CreateAccount actor={actor} fetchAccounts={fetchAccounts} />
            {accounts.map((account, index) => (
              <Account key={index} {...account} actor={actor} />
            ))}
            <button onClick={reset_account}>Reset Account</button>
          </div>
        ) : (
          <div
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center"
            }}
          >
            <input
              type="text"
              placeholder="Enter Canister id"
              value={canisterId}
              onChange={e => setCanisterId(e.target.value)}
            />
            <button onClick={installCanister}>Install Canister</button>
            <button onClick={createUser}>Create User</button>
          </div>
        )}
        {isAuthenticated && (
          <div
            style={{
              display: "flex",
              justifyContent: "center"
            }}
          >
            <button onClick={logout}>Logout</button>
          </div>
        )}
      </main>
      <Response response={status} />
      <Footer
        actor={actor}
        authClient={authClient}
        version={version}
        setError={setError}
        setLoading={setLoading}
        setVersion={setVersion}
      />
    </div>
  )
}

export default HomePage
