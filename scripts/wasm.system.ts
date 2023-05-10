import { ReleaseArgs } from "declarations/b3_system/b3_system.did"
import { B3System } from "../src/service/actor"
import { systemLocalActor } from "./actor"
import { loadWasm, readVersion } from "./utils"

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

const loadRelease = async (
  actor: B3System,
  wasmModule: number[],
  version: string
) => {
  console.log(`loading wasm code v${version} in System.`)

  console.log(`Wasm size:`, wasmModule.length)

  const release: ReleaseArgs = {
    version,
    features: [["add new system", "wasm system"]],
    size: BigInt(wasmModule.length)
  }

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await actor.load_release(chunks, release)

    console.log(`Chunks :`, result)
  }

  console.log(`loading done.`)
}

export const load = async (actor: B3System) => {
  const wasmModule = await loadWasm()
  const version = await readVersion()

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  await loadRelease(actor, wasmModule, version)
}
;(async () => {
  const actor = await systemLocalActor()

  await load(actor)
})()
