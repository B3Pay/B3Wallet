import { createReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { b3wallet, canisterId, idlFactory } from "declarations/b3wallet"

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
  withDevtools: true
})

export type WalletDynamicField = ReActorMethodField<B3Wallet>
