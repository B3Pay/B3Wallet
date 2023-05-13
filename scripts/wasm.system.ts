import { ReleaseArgs } from "declarations/b3_system/b3_system.did"
import { B3System } from "../src/service/actor"
import { systemLocalActor } from "./actor"
import { chunkGenerator, loadWasm, readVersion } from "./utils"

const loadRelease = async (
  actor: B3System,
  wasmModule: number[],
  version: string
) => {
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

  console.log(`Loading done.`)
}

export const load = async (actor: B3System, reload: boolean) => {
  const wasmModule = await loadWasm()
  const version = await readVersion()

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  if (reload) {
    console.log(`Reloading wasm code v${version} in System.`)

    await actor.remove_release(version)
  } else {
    console.log(`Loading wasm code v${version} in System.`)
  }

  await loadRelease(actor, wasmModule, version)
}

const loader = async (reload: boolean) => {
  const actor = await systemLocalActor()

  await load(actor, reload)
}

const reload =
  process.argv.find(arg => arg.indexOf("--reload") > -1) !== undefined

loader(reload)
