import { createReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { b3_system, canisterId, idlFactory } from "declarations/b3_system"

export type B3System = typeof b3_system

export const {
  useQueryCall: useSystemQuery,
  useUpdateCall: useSystemUpdate,
  useAuthClient: useSystemAuthClient,
  useActorStore: useSystemActorStore,
  useMethodFields: useSystemMethodFields
} = createReActor<B3System>({
  canisterId,
  idlFactory,
  withDevtools: true
})

export type SystemDynamicField = ReActorMethodField<B3System>
