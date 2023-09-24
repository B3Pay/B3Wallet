import { B3Wallet } from "../frontend/src/service"
import { walletActorIC, walletLocalActor } from "./actor"
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

export const load = async (name: string, actor: B3Wallet) => {
  const wasmModule = await loadWasm(name)
  const version = await readVersion(name)

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  await resetRelease(actor)
  await loadRelease(actor, wasmModule, version)
}

const loader = async (name: string, mainnet: boolean) => {
  const actor = await (mainnet ? walletActorIC : walletLocalActor)()

  await load(name, actor)
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
  } else if (!process.argv[i].startsWith("--")) {
    name = process.argv[i]
  }
}

console.log(`Network: ${mainnet}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

loader(name, mainnet)
