// load env from ../.env file
const envList = require("dotenv").config({ path: "../.env" }).parsed

console.log("envList", envList)

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
