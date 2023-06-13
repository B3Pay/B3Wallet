// copy user wasm from wasm/b3_wallet/b3_wallet.wasm to public/wasm/b3_wallet.wasm
import { existsSync, mkdirSync, readdirSync, statSync, writeFileSync } from "fs"
import { copyFile } from "fs/promises"
import path from "path"
import semver from "semver"
import { readVersion } from "./utils"

interface Release {
  name: string
  size: number
  url: string
  wasmFile: string
  version: string
}

const wasmPublicDirPath = path.join(__dirname, "../frontend", "public", "wasm")

const walletNames = ["b3_wallet", "b3_simple_wallet"]

const copy = async () => {
  for await (const walletName of walletNames) {
    const walletPath = path.join("wasm", walletName)

    for await (const fileName of readdirSync(walletPath, {
      withFileTypes: true
    })) {
      const src = path.join("wasm", walletName, fileName.name)

      await readVersion(walletName).then(async version => {
        const destFolder = path.join(
          wasmPublicDirPath,
          walletName,
          version || "latest"
        )

        if (!existsSync(destFolder)) {
          mkdirSync(destFolder, { recursive: true })
        }

        const dest = path.join(destFolder, fileName.name)

        await copyFile(src, dest)
      })
    }
  }

  // Get all folders in the directory
  const walletName = readdirSync(wasmPublicDirPath, { withFileTypes: true })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name)

  // Build the JSON object
  const result: Release[] = []

  // get folder name inside wasmPublicDirPath
  for await (const name of walletName) {
    const walletPath = path.join(wasmPublicDirPath, name)

    const walletVersion = readdirSync(walletPath, { withFileTypes: true })
      .filter(dirent => dirent.isDirectory())
      .map(dirent => dirent.name)

    for await (const version of walletVersion) {
      const versionPath = path.join(walletPath, version)
      const wasmFiles = readdirSync(versionPath).filter(
        file => path.extname(file) === ".wasm"
      )

      for (const wasmFile of wasmFiles) {
        const wasmFilePath = path.join(versionPath, wasmFile)
        const size = statSync(wasmFilePath).size

        const sizeInMb = (size / 1000000.0).toFixed(2)

        const url = path.join("wasm", name, version, wasmFile)

        console.log(`${version}/${wasmFile}: ${sizeInMb} MB`)

        result.push({
          url,
          name,
          size,
          wasmFile,
          version
        })
      }
      console.log("--------------------")
    }
  }

  result.sort((a, b) => semver.compare(b.version, a.version))

  // Convert the object to a JSON string
  const json = JSON.stringify(result, null, 2)
  const wamsJson = path.join(wasmPublicDirPath, "releases.json")

  // Write the JSON string to a file
  writeFileSync(wamsJson, json)
}

;(async () => {
  await copy()
})()
