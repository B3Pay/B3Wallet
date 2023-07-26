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

const frontendReleasesPath = path.join(
  __dirname,
  "../frontend",
  "public",
  "releases"
)

const walletNames = ["b3_wallet"]

const copy = async () => {
  for await (const walletName of walletNames) {
    const walletPath = path.join("wasm", walletName)

    const files = readdirSync(walletPath, {
      withFileTypes: true
    })

    for await (const file of files) {
      const src = path.join("wasm", walletName, file.name)

      await readVersion(walletName).then(async version => {
        const destFolder = path.join(
          frontendReleasesPath,
          walletName,
          version || "latest"
        )

        if (!existsSync(destFolder)) {
          mkdirSync(destFolder, { recursive: true })
        }

        const dest = path.join(destFolder, file.name)

        await copyFile(src, dest)
      })
    }
  }

  // Get the list of files
  const existingWallets = readdirSync(frontendReleasesPath, {
    withFileTypes: true
  })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name)

  // Build the JSON object
  const releases: Release[] = []

  // get folder name inside frontendReleasesPath
  for await (const name of existingWallets) {
    const walletPath = path.join(frontendReleasesPath, name)

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

        const url = path.join("releases", name, version, wasmFile)

        console.log(`${version}/${wasmFile}: ${sizeInMb} MB`)

        releases.push({
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

  releases.sort((a, b) => semver.compare(b.version, a.version))

  // Convert the object to a JSON string
  const json = JSON.stringify(releases, null, 2)
  const wamsJson = path.join(frontendReleasesPath, "index.json")

  // Write the JSON string to a file
  writeFileSync(wamsJson, json)
}

;(async () => {
  await copy()
})()
