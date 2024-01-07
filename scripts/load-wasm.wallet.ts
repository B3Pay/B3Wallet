import { callWalletMethod } from "./b3wallet"
import { chunkGenerator, hashToHex, loadWasmFile, readVersion } from "./utils"
import dfx from "../dfx.json"
import { updateAgent } from "./agent"

const loadWasmChunk = async (wasmModule: number[]) => {
  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await callWalletMethod("load_wasm", chunks)

    console.log("Chunks: ", result)
  }
}

export const load = async (appId: string, reload: boolean) => {
  const version = await readVersion(appId)

  if (!version) {
    console.error("Version for wasm cannot be read.")
    return
  }

  console.log(`Loading ${appId} wasmModule v${version} in WalletCanister.`)

  const { wasmModule, wasm_hash, wasm_size } = await loadWasmFile(appId)
  console.log("Wasm size:", wasm_size, "hash:", hashToHex(wasm_hash))

  if (reload) {
    try {
      await callWalletMethod("unload_wasm")
    } catch (e) {
      console.error("Error removing release:", appId, version)
    }
  }

  await loadWasmChunk(wasmModule)

  console.log("Wasm loaded.")
}

type AvailableAppIds = keyof typeof dfx.canisters

let appId: AvailableAppIds = "b3wallet"
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
    appId = process.argv[i] as AvailableAppIds
  }
}

console.log(`Network: ${mainnet ? "mainnet" : "local"}`) // Outputs: 'ic' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload
console.log(`Reload: ${reload}`) // Outputs: 'true' if you ran: ts-node main.ts renrk-eyaaa-aaaaa-aaada-cai --network=ic --reload

updateAgent(mainnet)
  .then(() => callWalletMethod("version"))
  .then(version => console.log("Wallet version:", version))
  .then(() => load(appId, reload))
  .catch(console.error)
