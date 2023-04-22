const DFXWebPackConfig = require("./dfx.webpack.config")

const envList = DFXWebPackConfig.initCanisterIds()

const webpack = require("webpack")

// Make DFX_NETWORK available to Web Browser with default "local" if DFX_NETWORK is undefined
const EnvPlugin = new webpack.EnvironmentPlugin(envList)

module.exports = {
  // eslint-disable-next-line no-unused-vars
  webpack: (config, { buildId, dev, isServer, defaultLoaders, webpack }) => {
    // Plugin
    config.plugins.push(EnvPlugin)

    // Important: return the modified config
    return config
  },
  output: "export"
}
