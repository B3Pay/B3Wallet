import { createReActor } from "@ic-reactor/react"
import { ExtractedFunction } from "@ic-reactor/store"
import { b3system, canisterId, idlFactory } from "@src/declarations/b3system"
import { agentManager } from "./agent"

export type B3System = typeof b3system

export const {
  useQueryCall: useSystemQuery,
  useUpdateCall: useSystemUpdate,
  useAuthClient: useSystemAuthClient,
  useActorStore: useSystemActorStore,
  useMethodFields: useSystemMethodFields
} = createReActor<B3System>({
  canisterId,
  idlFactory,
  withDevtools: true,
  agentManager
})

export type SystemDynamicField = ExtractedFunction<B3System>
