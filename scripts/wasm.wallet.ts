import { B3Wallet } from "../frontend/src/service/actor"
import { userLocalActor } from "./actor"
import { chunkGenerator, loadWasm, readVersion } from "./utils"

const resetRelease = (actor: B3Wallet) => actor.unload_wasm()

const loadRelease = async (
  actor: B3Wallet,
  wasmModule: number[],
  version: string
) => {
  console.log(`Loading wasm code ${version} in User Canister.`)

  console.log(`Wasm size:`, wasmModule.length)

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await actor.load_wasm(chunks)
    console.log(`Chunks :`, result)
  }

  console.log(`Loading done.`)
}

const load = async (actor: B3Wallet) => {
  const wasmModule = await loadWasm()
  const version = await readVersion()

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  await resetRelease(actor)
  await loadRelease(actor, wasmModule, version)
}

const loader = async (canisterId?: string) => {
  if (canisterId) {
    console.log(`Start Loading on Canister ID:`, canisterId)
  }

  const actor = await userLocalActor(canisterId)

  await load(actor)
}

const canisterId = process.argv[2] as string

loader(canisterId)
