import { ReleaseArgs } from "../frontend/declarations/b3_system/b3_system.did"
import { B3System } from "../frontend/src/service/actor"
import { systemActorIC, systemLocalActor } from "./actor"
import { chunkGenerator, loadWasm, readVersion } from "./utils"

const loadRelease = async (
  actor: B3System,
  wasmModule: number[],
  version: string
) => {
  console.log(`Wasm size:`, wasmModule.length)

  const release: ReleaseArgs = {
    version,
    features: [["", ""]],
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

  // loading candid version
  // const wasmModuleCandid = await loadWasm(true)
  // console.log(`Loading wasm code with candid v${version}-candid in System.`)
  // await loadRelease(actor, wasmModuleCandid, version + "-candid")
}

const loader = async (mainnet: boolean, reload: boolean) => {
  const actor = await (mainnet ? systemActorIC : systemLocalActor)()

  await load(actor, reload)
}

const reload =
  process.argv.find(arg => arg.indexOf("--reload") > -1) !== undefined

const mainnet =
  process.argv.find(arg => arg.indexOf("--mainnet") > -1) !== undefined

loader(mainnet, reload)
