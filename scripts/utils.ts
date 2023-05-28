import { Ed25519KeyIdentity } from "@dfinity/identity"
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1"

import { readFileSync } from "node:fs"
import { open, readFile } from "node:fs/promises"
// @ts-ignore
import pemfile from "pem-file"

/**
 * Source MOPS: https://github.com/ZenVoich/mops/blob/master/cli/pem.js
 * Forum: https://forum.dfinity.org/t/using-dfinity-agent-in-node-js/6169/60?u=peterparker
 */
const decode = (rawKey: string) => {
  const buf = pemfile.decode(rawKey)

  if (rawKey.includes("EC PRIVATE KEY")) {
    if (buf.length !== 118) {
      throw Error("expecting byte length 118 but got " + buf.length)
    }

    return Secp256k1KeyIdentity.fromSecretKey(buf.slice(7, 39))
  }

  if (buf.length !== 85) {
    throw Error("expecting byte length 85 but got " + buf.length)
  }

  const secretKey = Buffer.concat([buf.slice(16, 48), buf.slice(53, 85)])
  return Ed25519KeyIdentity.fromSecretKey(secretKey)
}

export const initIdentity = (mainnet: boolean) => {
  const file = `/Users/b3hr4d/.config/dfx/identity/${
    mainnet ? "main" : "default"
  }/identity.pem`
  const buffer = readFileSync(file)
  const key = buffer.toString("utf-8")

  return decode(key)
}

export const loadWasm = async (candid?: boolean) => {
  const buffer = await readFile(
    `${process.cwd()}/wasm/b3_wallet/b3_wallet${candid ? "_candid" : ""}.wasm`
  )
  return [...new Uint8Array(buffer)]
}

export const readVersion = async () => {
  const file = await open(`${process.cwd()}/backend/b3_wallet/Cargo.toml`)

  try {
    for await (const line of file.readLines()) {
      const version = line.match(/version = "(.*)"/)?.[1]

      if (version !== undefined) {
        return version
      }
    }

    return undefined
  } finally {
    await file.close()
  }
}

export const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}
