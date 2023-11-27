import createReActor from "@re-actor/core"
import { canisterId, createActor } from "declarations/b3_system"

export const {
  ReActorProvider: SystemProvider,
  callActor,
  useActorMethod,
  useAuthClient
} = createReActor(agent =>
  createActor(canisterId, {
    agent
  })
)
