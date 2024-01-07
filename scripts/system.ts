require("dotenv").config()
import { createReActorStore } from "@ic-reactor/store"
import { HttpAgent } from "@dfinity/agent"
import { b3system, canisterId, idlFactory } from "../src/declarations/b3system"
import { initIdentity } from "./utils"

export type B3System = typeof b3system

export const { actorStore, callMethod, agentManager } =
  createReActorStore<B3System>({
    canisterId,
    idlFactory
  })

export const loadSystemActor = async (mainnet: boolean) => {
  const identity = initIdentity(mainnet)
  console.log("Identity:", identity.getPrincipal().toText())

  try {
    const agent = new HttpAgent({
      host: mainnet ? "https://ic0.app" : "http://localhost:4943",
      identity
    })

    agentManager.updateAgent(agent)

    await new Promise<void>(resolve => {
      const unsubscribe = actorStore.subscribe(async state => {
        if (state.initialized) {
          unsubscribe()
          resolve()
        }
      })
    })

    const version = await callMethod("version")
    console.log("System Actor initialized. Version:", version)
  } catch (error) {
    console.error("System Actor initialization failed:", error)
  }
}
