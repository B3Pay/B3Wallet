import { ReleaseArgs } from "../frontend/declarations/b3_system/b3_system.did"
import { B3System } from "../frontend/src/service/actor"
import { systemActorIC, systemLocalActor } from "./actor"
import { chunkGenerator, loadWasm, readVersion } from "./utils"

const loadRelease = async (
  actor: B3System,
  wallet: string,
  wasmModule: number[],
  version: string
) => {
  console.log(`Wasm size:`, wasmModule.length)

  const release: ReleaseArgs = {
    name: wallet,
    version,
    features: [["", ""]],
    size: BigInt(wasmModule.length)
  }

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await actor.load_release(wallet, chunks, release)

    console.log(`Chunks :`, result)
  }

  console.log(`Loading done.`)
}

export const load = async (
  wallet: string,
  actor: B3System,
  candid: boolean,
  reload: boolean
) => {
  const wasmModule = await loadWasm(wallet, candid)
  const version = await readVersion(wallet)

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  const versionName = version + (candid ? "-candid" : "")

  console.log(`Loading ${wallet} wasm code v${versionName} in SystemCanister.`)

  if (reload) {
    try {
      await actor.remove_release(wallet, versionName)
    } catch (e) {
      console.error(`Error removing release:`, wallet, versionName)
    }
  }

  await loadRelease(actor, wallet, wasmModule, versionName)
}

const loader = async (
  wallets: string[],
  mainnet: boolean,
  candid: boolean,
  reload: boolean
) => {
  const actor = await (mainnet ? systemActorIC : systemLocalActor)()

  for await (const wallet of wallets) {
    await load(wallet, actor, candid, reload)
  }
}

let wallets: string[] = ["b3_wallet", "b3_simple_wallet"]
let mainnet: boolean = false
let reload: boolean = false
let candid: boolean = false

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i].startsWith("--network=")) {
    let network = process.argv[i].split("=")[1]
    if (network === "ic" || network === "mainnet") {
      mainnet = true
    }
  } else if (process.argv[i] === "--candid") {
    candid = true
  } else if (process.argv[i] === "--reload") {
    reload = true
  } else if (!process.argv[i].startsWith("--")) {
    wallets = [process.argv[i]]
  }
}

console.log(`Network: ${mainnet ? "mainnet" : "local"}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload
console.log(`Reload: ${reload}`) // Outputs: 'true' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

loader(wallets, mainnet, candid, reload)
