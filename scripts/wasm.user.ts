import { B3User } from "../src/service/actor"
import { userLocalActor } from "./actor"
import { loadWasm, readVersion } from "./utils"

const resetRelease = (actor: B3User) => actor.reset_wasm()

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

const loadRelease = async (
  actor: B3User,
  wasmModule: number[],
  version: string
) => {
  console.log(`loading wasm code ${version} in User Canister.`)

  console.log(`Wasm size:`, wasmModule.length)

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await actor.load_wasm(chunks, version)
    console.log(`Chunks :`, result)
  }

  console.log(`loading done.`)
}

const load = async (actor: B3User) => {
  const wasmModule = await loadWasm()
  const version = await readVersion()

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  await resetRelease(actor)
  await loadRelease(actor, wasmModule, version)
}

;(async () => {
  const actor = await userLocalActor()

  await load(actor)
})()
