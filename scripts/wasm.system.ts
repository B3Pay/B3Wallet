import { B3System } from "../src/service/actor"
import { systemLocalActor } from "./actor"
import { loadWasm, readVersion } from "./utils"

const resetRelease = (actor: B3System) => actor.reset_release()

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

const loadRelease = async ({ actor, wasmModule, version }: any) => {
  console.log(`loading wasm code v${version} in System.`)

  console.log(`Wasm size:`, wasmModule.length)

  const upload = async (chunks: any) => {
    const result = await actor.load_release(chunks, version)
    console.log(`Chunks :`, result)
  }

  for await (const chunks of chunkGenerator(wasmModule)) {
    await upload(chunks)
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

  await resetRelease(actor)
  await loadRelease({ actor, wasmModule, version })
}
;(async () => {
  const actor = await systemLocalActor()

  await load(actor)
})()
