// copy user wasm from .dfx/local/canisters/b3_wallet/b3_wallet.wasm to public/wasm/b3_wallet.wasm

import path from "path"

import { copyFile } from "fs/promises"

const copy = async () => {
  const src = path.join(".dfx/local/canisters/b3_wallet/", "b3_wallet.wasm")
  const dest = path.join("public", "wasm", "b3_wallet.wasm")

  await copyFile(src, dest)
}

;(async () => {
  await copy()
})()
