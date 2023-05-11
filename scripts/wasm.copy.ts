// copy user wasm from .dfx/local/canisters/b3_signer/b3_signer.wasm to public/wasm/b3_signer.wasm

import path from "path"

import { copyFile } from "fs/promises"

const copy = async () => {
  const src = path.join(".dfx/local/canisters/b3_signer/", "b3_signer.wasm")
  const dest = path.join("public", "wasm", "b3_signer.wasm")

  await copyFile(src, dest)
}

;(async () => {
  await copy()
})()
