// copy user wasm from wasm/b3_wallet/b3_wallet.wasm to public/wasm/b3_wallet.wasm

import path from "path"

import { copyFile } from "fs/promises"

const copy = async () => {
  const src = path.join("wasm/b3_wallet/", "b3_wallet.wasm")
  const dest = path.join("public", "wasm", "b3_wallet.wasm")

  await copyFile(src, dest)

  const src1 = path.join("wasm/b3_wallet/", "b3_wallet_candid.wasm")
  const dest1 = path.join("public", "wasm", "b3_wallet_candid.wasm")

  await copyFile(src1, dest1)
}

;(async () => {
  await copy()
})()
