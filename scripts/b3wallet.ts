import { createReActorStore } from "@ic-reactor/core"
import { agentManager } from "./agent"
import { b3wallet, canisterId, idlFactory } from "../src/declarations/b3wallet"

export type B3Wallet = typeof b3wallet

export const { callMethod: callWalletMethod } = createReActorStore<B3Wallet>({
  agentManager,
  canisterId,
  idlFactory,
  initializeOnCreate: false
})
