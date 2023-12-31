import { createReActorStore } from "@ic-reactor/store"
import {
  b3_wallet,
  canisterId,
  idlFactory
} from "../src/declarations/b3_wallet"
import { initIdentity } from "./utils"

export type B3Wallet = typeof b3_wallet

export const { actorStore, callMethod, initialize } =
  createReActorStore<B3Wallet>({
    canisterId,
    idlFactory,
    initializeOnMount: false
  })

export const loadSystemActor = async (mainnet: boolean) => {
  const identity = initIdentity(mainnet)
  console.log("Identity:", identity.getPrincipal().toText())

  initialize(
    {
      host: mainnet ? "https://ic0.app" : "http://localhost:4943",
      identity
    },
    !mainnet
  )

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
}
