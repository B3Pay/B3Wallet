// copy user wasm from .dfx/local/canisters/b3_user/b3_user.wasm to public/wasm/b3_user.wasm

import path from "path"

import { copyFile } from "fs/promises"

const copy = async () => {
  const src = path.join(".dfx/local/canisters/b3_user/", "b3_user.wasm")
  const dest = path.join("public", "wasm", "b3_user.wasm")

  await copyFile(src, dest)
}

;(async () => {
  await copy()
})()
