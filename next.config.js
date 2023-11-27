// Adjust the path to load env from ../.env file
const envList = require("dotenv").config({ path: "./.env.local" }).parsed || {}

// Adjust the path to get version from package.json
const { version } = require("./package.json")

envList.NEXT_PUBLIC_IC_HOST =
  envList.DFX_NETWORK === "ic" ? "https://icp-api.io" : "http://localhost:4943"

envList.NEXT_PUBLIC_VERSION = version

/** @type {import('next').NextConfig} */
module.exports = {
  env: envList,
  output: "export",
  images: {
    unoptimized: true,
  },
  staticPageGenerationTimeout: 10000
}
