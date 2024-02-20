import { createReActorStore } from "@ic-reactor/core"
import { b3system, canisterId, idlFactory } from "../src/declarations/b3system"
import { agentManager } from "./agent"

export type B3System = typeof b3system

export const { callMethod: callSystemMethod } = createReActorStore<B3System>({
  canisterId,
  idlFactory,
  agentManager,
  initializeOnCreate: false
})
