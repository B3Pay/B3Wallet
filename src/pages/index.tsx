/* eslint-disable @next/next/no-img-element */
import { Principal } from "@dfinity/principal"
import CreateAccount from "components/CreateAccount"
import EthAccount from "components/EthAccount"
import { Response } from "components/Response"
import {
  CanisterStatus,
  SignerAccount
} from "declarations/b3_signer/b3_signer.did"
import useAuthClient from "hooks/useAuthClient"
import Head from "next/head"
import { useCallback, useEffect, useState } from "react"
import { B3User, makeB3UserActor } from "service/actor"
import styles from "styles/Home.module.css"

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

export const loadRelease = async (
  actor: B3User,
  wasmModule: number[],
  version: string
) => {
  console.log(`loading wasm code ${version} in User Canister.`)

  console.log(`Wasm size:`, wasmModule.length)

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await actor.load_wasm(chunks)
    console.log(`Chunks :`, result)
  }

  console.log(`loading done.`)
}

function HomePage() {
  const { isAuthenticated, authClient, login, logout, systemActor } =
    useAuthClient()

  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string>()

  const [status, setStatus] = useState<CanisterStatus>()
  const [accounts, setAccounts] = useState<SignerAccount[]>([])
  const [actor, setActor] = useState<B3User>()
  const [canisterId, setCanisterId] = useState<string>("")

  const [version, setVersion] = useState<string>("")

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      if (!canisterId || !authClient) {
        console.log("no canisterId or authClient")
        return
      }
      setLoading(true)

      const userActor = makeB3UserActor(canisterId, authClient.getIdentity())

      const status = await userActor.status()

      setStatus(status)

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

    const [control] = await systemActor.get_signer()

    console.log(control)

    if (!control) {
      setError("No user control found")
      setLoading(false)
      return
    }

    const canisterId = control.canister_id.toString()

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

  const createUser = async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)
    const userControl = await systemActor.create_signer_canister()

    fetchUserActor(userControl.canister_id.toString())
    setLoading(false)
  }

  const installCanister = async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(true)

    const canisterPrincipal = Principal.fromText(canisterId)

    const userControl = await systemActor.install_signer_canister(
      canisterPrincipal
    )

    fetchUserActor(userControl.canister_id.toString())
    setLoading(false)
  }

  const updateCanisterWasm = async () => {
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(true)

    const wasm = await fetch("wasm/b3_signer.wasm")

    const wasm_buffer = await wasm.arrayBuffer()
    const wasm_module = Array.from(new Uint8Array(wasm_buffer))

    const result = await actor.reset_wasm()
    console.log(result)

    await loadRelease(actor, wasm_module, "0.0.0-alpha.4")

    console.log("Wasm loaded")

    setVersion(version)
    setLoading(false)
  }

  const upgradeCanister = async () => {
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    const wasm_version = await actor.wasm_hash()

    console.log("Wasm version:", wasm_version)

    if (!wasm_version || wasm_version === version) {
      console.log("Canister already upgraded")
      return
    }

    setLoading(true)

    try {
      await actor.upgrade_canister()
    } catch (e) {
      console.log(e)
    }

    console.log("Canister upgraded")

    const current_version = await actor.version()

    setVersion(current_version)

    setLoading(false)
  }

  const reset_account = async () => {
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(true)

    const result = await actor.reset_accounts()

    console.log(result)

    fetchAccounts()

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
            <label>Accounts: &nbsp;</label>
            {accounts.map((account, index) => (
              <EthAccount key={index} {...account} actor={actor} />
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
          <section
            style={{
              display: "flex",
              justifyContent: "center"
            }}
          >
            <button onClick={logout}>Logout</button>
          </section>
        )}
      </main>
      <Response response={status} />
      <footer
        className={styles.footer}
        style={{
          display: "flex",
          justifyContent: "space-between"
        }}
      >
        <p>Version: {version}</p>
        <button onClick={updateCanisterWasm}>Load Wasm</button>
        <button onClick={upgradeCanister}>Upgrade Canister</button>
      </footer>
    </div>
  )
}

export default HomePage
