"use client"
import createReActor from "@re-actor/core"
import { canisterId, createActor } from "declarations/b3_system"

export const {
  initialize: initializeSystem,
  ReActorProvider,
  callActor: callSystem,
  useActorMethod: useSystemMethod,
  useAuthClient: useSystemAuthClient
} = createReActor(agent =>
  createActor(canisterId, {
    agent
  })
)

export default ReActorProvider
