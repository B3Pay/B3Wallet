import {
  CreateAppArgs,
  CreateReleaseArgs
} from "../src/declarations/b3system/b3system.did"
import { callSystemMethod } from "./b3system"
import {
  chunkGenerator,
  hashToHex,
  loadImageFile,
  loadWasmFile,
  readVersion
} from "./utils"
import dfx from "../dfx.json"
import { updateAgent } from "./agent"

interface Metadata {
  name: string
  repo: string
  logo: string
  description: string
}

async function createApp(appId: AvailableAppIds) {
  const metadataJson: Metadata = require(`../canisters/${appId}/metadata.json`)

  const metadata: CreateAppArgs["metadata"] = Object.entries(
    metadataJson
  ).reduce((acc, [key, value]) => {
    if (key === "logo") {
      acc.push([
        "logo",
        {
          Blob: loadImageFile(value, false)
        }
      ])
      return acc
    }

    acc.push([key, { Text: value }])
    return acc
  }, [] as CreateAppArgs["metadata"])

  return await callSystemMethod("create_app", {
    name: appId,
    description: metadataJson.description,
    metadata
  })
}

async function addRelease(
  app_id: string,
  version: string,
  wasm_hash: number[],
  size: bigint
) {
  const release: CreateReleaseArgs = {
    app_id,
    version,
    wasm_hash,
    features: "",
    size
  }

  return await callSystemMethod("add_release", release)
}

const loadWasmChunk = async (wasm_hash: number[], wasmModule: number[]) => {
  for await (const chunks of chunkGenerator(wasmModule)) {
    const result = await callSystemMethod("load_wasm_chunk", wasm_hash, chunks)

    console.log("Chunks: ", result)
  }
}

export const load = async (appId: string, reload: boolean) => {
  const version = await readVersion(appId)

  if (!version) {
    console.error("Version for wasm cannot be read.")
    return
  }

  console.log(`Loading ${appId} wasmModule v${version} in SystemCanister.`)

  const { wasmModule, wasm_hash, wasm_size } = await loadWasmFile(appId)
  console.log("Wasm size:", wasm_size, "hash:", hashToHex(wasm_hash))

  if (reload) {
    try {
      await callSystemMethod("remove_release", wasm_hash)
    } catch (e) {
      console.error("Error removing release:", appId, version)
    }
  }

  const release = await addRelease(appId, version, wasm_hash, wasm_size)

  console.log("Release added:", release)

  await loadWasmChunk(wasm_hash, wasmModule)

  console.log("Wasm loaded.")

  const app = await callSystemMethod("get_app", appId)

  console.log("App:", app)
}

const loader = async (appId: AvailableAppIds, reload: boolean) => {
  const app = await callSystemMethod("get_app", appId)

  if ("Err" in app) {
    const appView = await createApp(appId)
    if ("Err" in appView) {
      console.error("Error creating app:", appView.Err)
      return
    } else {
      appId = appView.Ok.app_id as AvailableAppIds
      console.log("App created:", appView)
    }
  } else {
    appId = app.Ok.app_id as AvailableAppIds
    console.log("App exists:", app)
  }

  await load(appId, reload)
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
  .then(() => callSystemMethod("version"))
  .then(version => console.log("System version:", version))
  .then(() => loader(appId, reload))
