import { ReleaseArgs } from "../frontend/declarations/b3_system/b3_system.did"
import { B3System } from "../frontend/src/service/actor"
import { systemActorIC, systemLocalActor } from "./actor"
import { chunkGenerator, loadWasm, readVersion } from "./utils"

const loadRelease = async (
  actor: B3System,
  name: string,
  wasmModule: number[],
  version: string
) => {
  console.log(`Wasm size:`, wasmModule.length)

  const release: ReleaseArgs = {
    name,
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

export const load = async (name: string, actor: B3System, reload: boolean) => {
  const wasmModule = await loadWasm(name)
  const version = await readVersion(name)

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

  await loadRelease(actor, name, wasmModule, version)

  // loading candid version
  const wasmModuleCandid = await loadWasm(name, true)
  console.log(`Loading wasm code with candid v${version}-candid in System.`)
  await loadRelease(actor, name, wasmModuleCandid, version + "-candid")
}

const loader = async (name: string, mainnet: boolean, reload: boolean) => {
  const actor = await (mainnet ? systemActorIC : systemLocalActor)()

  await load(name, actor, reload)
}

let name: string = "b3_wallet"
let mainnet: boolean = false
let reload: boolean = false

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i].startsWith("--network=")) {
    let network = process.argv[i].split("=")[1]
    if (network === "ic" || network === "mainnet") {
      mainnet = true
    }
  } else if (process.argv[i] === "--reload") {
    reload = true
  } else if (!process.argv[i].startsWith("--")) {
    name = process.argv[i]
  }
}

console.log(`Network: ${mainnet}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload
console.log(`Reload: ${reload}`) // Outputs: 'true' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

loader(name, mainnet, reload)
