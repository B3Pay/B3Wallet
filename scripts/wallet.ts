require("dotenv").config()
import { createReActorStore } from "@ic-reactor/store"
import { b3wallet, canisterId, idlFactory } from "../src/declarations/b3wallet"
import { initIdentity } from "./utils"

export type B3Wallet = typeof b3wallet

export const { actorStore, callMethod, agentManager } =
  createReActorStore<B3Wallet>({
    canisterId,
    idlFactory,
    initializeOnCreate: false
  })

export const loadWalletActor = async (mainnet: boolean) => {
  const identity = initIdentity(mainnet)
  console.log("Identity:", identity.getPrincipal().toText())

  try {
    await agentManager.updateAgent({
      host: mainnet ? "https://ic0.app" : "http://127.0.0.1:4943",
      identity
    })

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
