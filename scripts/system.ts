require("dotenv").config()
import { createReActorStore } from "@ic-reactor/store"
import { b3system, canisterId, idlFactory } from "../src/declarations/b3system"
import { initIdentity } from "./utils"

export type B3System = typeof b3system

export const { actorStore, callMethod, agentManager } =
  createReActorStore<B3System>({
    canisterId,
    idlFactory,
    initializeOnCreate: false
  })

export const loadSystemActor = async (mainnet: boolean) => {
  const identity = initIdentity(mainnet)
  console.log("Identity:", identity.getPrincipal().toText())

  try {
    await agentManager.updateAgent({
      host: mainnet ? "https://ic0.app" : "http://127.0.0.1:4943",
      identity
    })

    const version = await callMethod("version")
    console.log("System Actor initialized. Version:", version)
  } catch (error) {
    console.error("System Actor initialization failed:", error)
  }
}
