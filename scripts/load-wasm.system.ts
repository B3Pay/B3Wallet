import {
  CreateReleaseArgs,
  Value
} from "../src/declarations/b3_system/b3_system.did"
import { callMethod, loadSystemActor } from "./system"
import { chunkGenerator, hashToHex, loadWasmFile, readVersion } from "./utils"

async function createApp(name: string) {
  const repo: Value = {
    Text: "https://github.com/B3Pay/b3-wallet"
  }

  return await callMethod("create_app", {
    name,
    description: "Decentralized wallet for the Internet Computer",
    metadata: [["repo", repo]]
  })
}

async function addRelease(
  name: string,
  version: string,
  wasm_hash: number[],
  size: bigint
) {
  const release: CreateReleaseArgs = {
    id: name,
    version,
    wasm_hash,
    features: "",
    size
  }

  return await callMethod("add_release", name, release)
}

const loadWasmChunk = async (wasm_hash: number[], wasmModule: number[]) => {
  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await callMethod("load_wasm_chunk", wasm_hash, chunks)

    console.log("Chunks: ", result)
  }

  console.log("Loading done.")
}

export const load = async (name: string, reload: boolean) => {
  const version = await readVersion(name)

  if (!version) {
    console.error("Version for wasm cannot be read.")
    return
  }

  console.log(`Loading ${name} wasmModule v${version} in SystemCanister.`)

  const { wasmModule, wasm_hash, wasm_size } = await loadWasmFile(name)
  console.log("Wasm size:", wasm_size, "hash:", hashToHex(wasm_hash))

  if (reload) {
    try {
      await callMethod("remove_release", wasm_hash)
    } catch (e) {
      console.error("Error removing release:", name, version)
    }
  }

  const release = await addRelease(name, version, wasm_hash, wasm_size)

  console.log("Release added:", release)

  await loadWasmChunk(wasm_hash, wasmModule)
}

const loader = async (name: string, mainnet: boolean, reload: boolean) => {
  await loadSystemActor(mainnet)

  const app = await callMethod("get_app", name)

  if ("Err" in app) {
    const appView = await createApp(name)
    console.log("App created:", appView)
  }

  await load(name, reload)
}

let app = "b3_wallet"
let mainnet: boolean = false
let reload: boolean = false

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i].startsWith("--network=")) {
    const network = process.argv[i].split("=")[1]
    if (network === "ic" || network === "mainnet") {
      mainnet = true
    }
  } else if (process.argv[i] === "--reload") {
    reload = true
  } else if (!process.argv[i].startsWith("--")) {
    app = process.argv[i]
  }
}

console.log(`Network: ${mainnet ? "mainnet" : "local"}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload
console.log(`Reload: ${reload}`) // Outputs: 'true' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

loader(app, mainnet, reload)
