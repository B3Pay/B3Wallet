"use client"
import createReActor from "@re-actor/core"
import { canisterId, createActor } from "declarations/b3_system"

export const {
  ReActorProvider: SystemProvider,
  callActor: callSystem,
  useActorMethod: useSystemMethod,
  useAuthClient: useSystemAuthClient
} = createReActor(agent =>
  createActor(canisterId, {
    agent
  })
)
