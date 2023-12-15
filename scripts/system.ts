import { createReActorStore } from "@ic-reactor/store"
import { canisterId, createActor } from "../src/declarations/b3_system"

export const { callMethod, initialize } = createReActorStore(agent => {
  return createActor(canisterId, {
    agent
  })
})

export type B3System = ReturnType<typeof createActor>
