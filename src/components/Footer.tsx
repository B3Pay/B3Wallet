import React from "react"
import { B3User } from "service/actor"

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

interface FooterProps {
  actor?: B3User
  authClient?: any
  version: string
  setVersion: React.Dispatch<React.SetStateAction<string>>
  setLoading: React.Dispatch<React.SetStateAction<boolean>>
  setError: React.Dispatch<React.SetStateAction<string | undefined>>
}

export const Footer: React.FC<FooterProps> = ({
  actor,
  authClient,
  setLoading,
  setError,
  setVersion,
  version
}) => {
  const updateCanisterWasm = async () => {
    setError(undefined)
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(true)

    const wasm = await fetch("wasm/b3_wallet.wasm")

    const wasm_buffer = await wasm.arrayBuffer()
    const wasm_module = Array.from(new Uint8Array(wasm_buffer))

    await loadRelease(actor, wasm_module, "0.0.0-alpha.1")

    console.log("Wasm loaded")

    setVersion(version)
    setLoading(false)
  }

  const upgradeCanister = async () => {
    setError(undefined)
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    const wasm_version = await actor.wasm_hash_string()

    console.log("Wasm version:", wasm_version)

    if (!wasm_version || wasm_version === version) {
      console.log("Canister already upgraded")
      return
    }

    setLoading(true)

    try {
      await actor.upgrage_wallet()
    } catch (e) {
      console.log(e)
    }

    console.log("Canister upgraded")

    const current_version = await actor.version()

    setVersion(current_version)

    setLoading(false)
  }
  return (
    <footer
      style={{
        display: "flex",
        justifyContent: "space-between"
      }}
    >
      <p>Version: {version}</p>
      <button onClick={updateCanisterWasm}>Load Wasm</button>
      <button onClick={upgradeCanister}>Upgrade</button>
    </footer>
  )
}
