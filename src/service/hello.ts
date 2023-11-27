import createReActor from "@re-actor/core"
import { canisterId, createActor } from "declarations/hello"

export const {
  ReActorProvider,
  callActor,
  initialize,
  useReActor,
  useActorState,
  useActorMethod,
  useAuthClient
} = createReActor(
  agent =>
    createActor(canisterId, {
      agent
    }),
  {
    host: process.env.NEXT_PUBLIC_IC_HOST
  }
)
