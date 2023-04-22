const path = require("path")
const writeFileSync = require("fs").writeFileSync

let localCanisters, prodCanisters, canisters

function initCanisterIds() {
  try {
    localCanisters = require(path.resolve(".dfx", "local", "canister_ids.json"))
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production")
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"))
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local")
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local")

  console.info(`initCanisterIds: network=${network}`)
  console.info(`initCanisterIds: DFX_NETWORK=${process.env.DFX_NETWORK}`)

  canisters = network === "local" ? localCanisters : prodCanisters

  const envList = {
    DFX_NETWORK: "local"
  }

  for (const canister in canisters) {
    const name = canister.toUpperCase()
    const address = canisters[canister][network]

    envList[`${name}_CANISTER_ID`] = address
  }

  writeFileSync(
    path.resolve(".env.local"),
    Object.entries(envList)
      .map(([key, value]) => `${key}=${value}`)
      .join("\n")
  )

  return envList
}

module.exports = {
  initCanisterIds: initCanisterIds
}
