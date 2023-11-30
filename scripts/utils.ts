import { Ed25519KeyIdentity } from "@dfinity/identity"
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1"

import { readFileSync } from "node:fs"
import { open, readFile } from "node:fs/promises"
import path from "path"

const pemfile = require("pem-file")
const os = require("os")

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

  const secretKey = Buffer.concat([buf.slice(16, 48)])
  return Ed25519KeyIdentity.fromSecretKey(secretKey)
}

export const initIdentity = (mainnet: boolean) => {
  const userHomeDir = os.homedir()
  const file = path.join(
    userHomeDir,
    `.config/dfx/identity/${mainnet ? "default" : "default"}/identity.pem`
  )
  const buffer = readFileSync(file)
  const key = buffer.toString("utf-8")

  return decode(key)
}

export const loadWasm = async (name: string, withCandid?: boolean) => {
  const buffer = await readFile(
    `${process.cwd()}/wasm/${name}/${name}${withCandid ? "_candid" : ""}.wasm`
  )
  console.log(
    "Wasm size:",
    buffer.length,
    "hash:",
    await calculateSha256(buffer.buffer)
  )

  return [...new Uint8Array(buffer)]
}

export const readVersion = async (name: string) => {
  const file = await open(`${process.cwd()}/backend/${name}/Cargo.toml`)

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

export async function calculateSha256(arrayBuffer: ArrayBuffer, asHex = true) {
  // Use the SubtleCrypto interface to perform the SHA-256 hash
  const hashBuffer = await crypto.subtle.digest("SHA-256", arrayBuffer)

  // Convert the hash to a hexadecimal string for easy comparison and display
  const hashArray = Array.from(new Uint8Array(hashBuffer))

  if (!asHex) {
    return hashArray
  }

  const hashHex = hashArray
    .map(byte => byte.toString(16).padStart(2, "0"))
    .join("")

  return hashHex
}

export async function calculateWasmHash(name: string, asHex: boolean) {
  const buffer = await readFile(`${process.cwd()}/wasm/${name}/${name}.wasm`)

  const hashBuffer = await crypto.subtle.digest("SHA-256", buffer.buffer)

  const hashArray = Array.from(new Uint8Array(hashBuffer))

  if (!asHex) {
    return hashArray
  }

  const hashHex = hashArray
    .map(byte => byte.toString(16).padStart(2, "0"))
    .join("")

  return hashHex
}
