import { CreateReleaseArgs } from "../src/declarations/b3_system/b3_system.did"
import { localAgent } from "./actor"
import { callMethod, initialize } from "./system"
import {
  calculateWasmHash,
  chunkGenerator,
  loadWasm,
  readVersion
} from "./utils"

const loadRelease = async (
  name: string,
  wasm_hash: number[],
  wasmModule: number[],
  version: string
) => {
  const release: CreateReleaseArgs = {
    id: name,
    version,
    wasm_hash,
    features: "",
    size: BigInt(wasmModule.length)
  }

  await callMethod("add_release", name, release)

  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await callMethod("load_wasm", wasm_hash, chunks)

    console.log(`Chunks :`, result)
  }

  console.log(`Loading done.`)
}

export const load = async (name: string, candid: boolean, reload: boolean) => {
  const wasmModule = await loadWasm(name, candid)
  const version = await readVersion(name)
  const wasm_hash = (await calculateWasmHash(name, false)) as number[]

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  const versionName = version + (candid ? "-candid" : "")

  console.log(`Loading ${name} wasm code v${versionName} in SystemCanister.`)

  if (reload) {
    try {
      await callMethod("remove_release", wasm_hash)
    } catch (e) {
      console.error(`Error removing release:`, name, versionName)
    }
  }

  await loadRelease(name, wasm_hash, wasmModule, versionName)
}

const loader = async (
  apps: string[],
  mainnet: boolean,
  candid: boolean,
  reload: boolean
) => {
  initialize({ host: mainnet ? "https://ic0.app" : "http://localhost:8000" })
  const { agent, identity } = await localAgent()

  initialize({
    host: mainnet ? "https://ic0.app" : "http://localhost:4943",
    identity
  })

  for await (const app of apps) {
    await load(app, candid, reload)
  }
}

let apps: string[] = ["b3_wallet"]
let mainnet: boolean = false
let reload: boolean = false
let candid: boolean = false

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i].startsWith("--network=")) {
    const network = process.argv[i].split("=")[1]
    if (network === "ic" || network === "mainnet") {
      mainnet = true
    }
  } else if (process.argv[i] === "--candid") {
    candid = true
  } else if (process.argv[i] === "--reload") {
    reload = true
  } else if (!process.argv[i].startsWith("--")) {
    apps = [process.argv[i]]
  }
}

console.log(`Network: ${mainnet ? "mainnet" : "local"}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload
console.log(`Reload: ${reload}`) // Outputs: 'true' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

loader(apps, mainnet, candid, reload)
