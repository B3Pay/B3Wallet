import { Button, Flex, Stack, Text } from "@chakra-ui/react"
import { useCallback, useState } from "react"
import { B3Wallet } from "service/actor"
import Error from "../../Error"

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

export const loadRelease = async (
  actor: B3Wallet,
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

interface WasmProps {
  actor: B3Wallet
  version: string
  setLoading: (loading: boolean) => void
}

const Wasm: React.FC<WasmProps> = ({
  actor,
  setLoading,

  version
}) => {
  const [error, setError] = useState<string>()

  const updateCanisterWasm = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    const wasm = await fetch("wasm/b3_wallet_candid.wasm")

    const wasm_buffer = await wasm.arrayBuffer()
    const wasm_module = Array.from(new Uint8Array(wasm_buffer))

    await loadRelease(actor, wasm_module, "0.0.0-alpha.2")

    console.log("Wasm loaded")

    setLoading(false)
  }, [actor, setLoading])

  const upgradeCanister = async () => {
    setError(undefined)

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

    setLoading(false)
  }

  const resetWasm = async () => {
    setError(undefined)

    setLoading(true)

    try {
      await actor.unload_wasm()
    } catch (e) {
      console.log(e)
    }

    console.log("Canister reset")

    setLoading(false)
  }

  return (
    <Stack>
      {error && <Error error={error} />}
      <Flex justify="space-between" align="center" h="50px">
        <Text>{version}</Text>
        <Button onClick={resetWasm}>Reset</Button>
        <Button onClick={updateCanisterWasm}>Load Wasm</Button>
        <Button onClick={upgradeCanister}>Upgrade</Button>
      </Flex>
    </Stack>
  )
}

export default Wasm
