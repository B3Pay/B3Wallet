import { Button, Flex, Text } from "@chakra-ui/react"
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
  setLoading: (loading: boolean) => void
  setVersion: React.Dispatch<React.SetStateAction<string>>
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

    const wasm = await fetch("wasm/b3_wallet_candid.wasm")

    const wasm_buffer = await wasm.arrayBuffer()
    const wasm_module = Array.from(new Uint8Array(wasm_buffer))

    await loadRelease(actor, wasm_module, "0.0.0-alpha.2")

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

  const resetWasm = async () => {
    setError(undefined)
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(true)

    try {
      await actor.unload_wasm()
    } catch (e) {
      console.log(e)
    }

    console.log("Canister reset")

    const current_version = await actor.version()

    setVersion(current_version)

    setLoading(false)
  }

  return actor ? (
    <Flex justify="space-between" align="center" w="100%" h="100px" padding={2}>
      <Text>{version}</Text>
      <Button onClick={resetWasm}>Reset</Button>
      <Button onClick={updateCanisterWasm}>Load Wasm</Button>
      <Button onClick={upgradeCanister}>Upgrade</Button>
    </Flex>
  ) : (
    <Flex justify="space-between" align="center" w="100%" h="100px" padding={2}>
      <Text>{version}</Text>
    </Flex>
  )
}
