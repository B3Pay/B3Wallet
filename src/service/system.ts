"use client"
import createReActor from "@ic-reactor/react"
import { canisterId, createActor } from "declarations/b3_system"

export const {
  initialize: initializeSystem,
  ReActorProvider,
  useQueryCall: useSystemQuery,
  useUpdateCall: useSystemUpdate,
  useAuthClient: useSystemAuthClient
} = createReActor(agent =>
  createActor(canisterId, {
    agent
  })
)

export default ReActorProvider
