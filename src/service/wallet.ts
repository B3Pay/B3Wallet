import { createReActor } from "@ic-reactor/react"
import { ExtractedFunction } from "@ic-reactor/store"
import { b3wallet, canisterId, idlFactory } from "@src/declarations/b3wallet"
import { agentManager } from "./agent"

export type B3Wallet = typeof b3wallet

export const {
  useQueryCall: useWalletQuery,
  useUpdateCall: useWalletUpdate,
  useAuthClient: useWalletAuthClient,
  useActorStore: useWalletActorStore,
  useMethodFields: useWalletMethodFields
} = createReActor<B3Wallet>({
  canisterId,
  idlFactory,
  withDevtools: true,
  agentManager
})

export type WalletDynamicField = ExtractedFunction<B3Wallet>
