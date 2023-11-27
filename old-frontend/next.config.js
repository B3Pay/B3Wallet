// load env from ../.env file
const envList = require("dotenv").config({ path: "../.env" }).parsed
// get version from package.json
const { version } = require("../package.json")

envList.NEXT_PUBLIC_IC_HOST =
  envList.DFX_NETWORK === "ic" ? "https://ic0.app" : "http://localhost:4943"

console.log("network", envList.DFX_NETWORK)

envList.NEXT_PUBLIC_VERSION = version

const webpack = require("webpack")

/** @type {import('next').NextConfig} */
module.exports = {
  webpack: config => {
    config.plugins = [...config.plugins, new webpack.EnvironmentPlugin(envList)]

    return config
  },
  output: "export",
  staticPageGenerationTimeout: 10000
}
