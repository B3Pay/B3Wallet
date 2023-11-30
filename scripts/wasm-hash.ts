import { calculateWasmHash } from "./utils"

const call = async (name: string, asHex: boolean) => {
  const hash = await calculateWasmHash(name, asHex)

  console.log(hash)
}

let name: string = "b3_wallet"
let asHex: boolean = false

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i] === "--help") {
    console.log("Usage: node wasm-hash.ts [name] [--hex]")
    console.log("  name: name of the wasm file without the extension")
    console.log("  --hex: output the hash as a hex string")
    process.exit(0)
  } else if (process.argv[i] === "--hex") {
    asHex = true
  } else if (!process.argv[i].startsWith("--")) {
    name = process.argv[i]
  }
}

call(name, asHex)
