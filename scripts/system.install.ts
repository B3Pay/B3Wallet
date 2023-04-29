#!/usr/bin/env node
import { B3System } from "../src/service/actor"
import { consoleSystemLocal } from "./actor"
import { loadWasm, readVersion } from "./utils"

const resetRelease = (actor: B3System) => actor.reset_release()

const installRelease = async ({ actor, wasmModule, version }: any) => {
  console.log(`Installing wasm code v${version} in console.`)

  console.log(`Wasm size:`, wasmModule.length)

  const chunkSize = 700000

  const upload = async (chunks: any) => {
    const result = await actor.load_release(chunks, version)
    console.log(`Chunks :`, result)
  }

  const chunkGenerator = async function* () {
    for (let start = 0; start < wasmModule.length; start += chunkSize) {
      yield wasmModule.slice(start, start + chunkSize)
    }
  }

  for await (const chunks of chunkGenerator()) {
    await upload(chunks)
  }

  console.log(`Installation done.`)
}

const install = async (actor: B3System) => {
  const wasmModule = await loadWasm()
  const version = await readVersion()

  if (!version) {
    console.error(`Version for wasm cannot be read.`)
    return
  }

  await resetRelease(actor)
  await installRelease({ actor, wasmModule, version })
}

;(async () => {
  const actor = await consoleSystemLocal()

  await install(actor)
})()
