require("dotenv").config()

import { createAgentManager } from "@ic-reactor/core"
import { initIdentity } from "./utils"

export const agentManager = createAgentManager({
  host: process.env.NEXT_PUBLIC_IC_HOST || "http://127.0.0.1:4943"
})

export const updateAgent = async (mainnet: boolean) => {
  const identity = initIdentity(mainnet)
  console.log("Identity:", identity.getPrincipal().toText())

  try {
    await agentManager.updateAgent({
      host: mainnet ? "https://ic0.app" : "http://127.0.0.1:4943",
      identity
    })
  } catch (error) {
    console.error("System Actor initialization failed:", error)
  }
}
