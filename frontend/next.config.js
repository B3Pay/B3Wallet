// load env from ../.env file
const envList = require("dotenv").config({ path: "../.env" }).parsed
// get version from package.json
const { version } = require("../package.json")

console.log("version", version)

envList.NEXT_PUBLIC_IC_HOST =
  envList.DFX_NETWORK === "ic" ? "https://ic0.app" : "http://localhost:8080"

console.log("network", envList.DFX_NETWORK)

envList.NEXT_PUBLIC_VERSION = version

const webpack = require("webpack")

// Make DFX_NETWORK available to Web Browser with default "local" if DFX_NETWORK is undefined
const EnvPlugin = new webpack.EnvironmentPlugin(envList)

module.exports = {
  // eslint-disable-next-line no-unused-vars
  webpack: config => {
    config.plugins.push(EnvPlugin)

    return config
  },
  output: "export"
}
